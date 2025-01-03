use std::net::IpAddr;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self};
use tokio::runtime::Runtime;

use clap::Parser;

// Define a struct to parse and hold command-line arguments
#[derive(Debug, Parser)]
struct Args {
    /// IP address of the port scan.
    #[arg(conflicts_with("cidr"), required_unless_present("cidr"))]
    addr: Option<IpAddr>, // Single IP address to scan

    #[arg(long)]
    cidr: Option<cidr::IpCidr>, // CIDR block for scanning multiple addresses

    /// Start of the range.
    #[arg(short = 's', long, default_value_t = 4000)]
    port_start: u16, // Start of the port range

    /// End of the range of ports to scan (inclusive).
    #[arg(short = 'e', long, default_value_t = 50000)]
    port_end: u16, // End of the port range
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();

    // Ensure the start port is non-zero and less than or equal to the end port
    assert!(args.port_start != 0);
    assert!(args.port_start <= args.port_end);

    // Create a new Tokio runtime
    let rt = Runtime::new()?;

    // Create a channel for sending scan results
    let (tx, mut rx) = mpsc::channel(10);

    rt.block_on(async {
        // Calculate the number of tasks required for the network range
        let n_tasks_per_network = (args.port_end - args.port_start) as usize;
        let mut tasks: Vec<_> = Vec::with_capacity(n_tasks_per_network);

        // Declare iterators for addresses from a single IP or a CIDR block
        let (mut from_single, mut from_cidr);

        // Determine the set of addresses to scan based on input
        let addrs: &mut dyn Iterator<Item = IpAddr> = if let Some(addr) = args.addr {
            from_single = vec![addr].into_iter();
            &mut from_single
        } else if let Some(network) = args.cidr {
            from_cidr = network.iter().map(|net| net.address());
            &mut from_cidr
        } else {
            unreachable!() // This should never happen due to argument validation
        };

        // Iterate over all addresses and ports to create scan tasks
        for addr in addrs {
            println!("? {addr}:{}-{}", args.port_start, args.port_end);
            for port in args.port_start..=args.port_end {
                let tx = tx.clone();
                let task = tokio::spawn(async move {
                    // Perform the scan and handle errors
                    if let Err(err) = scan(addr, port, tx).await {
                        eprintln!("error: {err}")
                    };
                });

                tasks.push(task);
            }
        }

        // Await the completion of all tasks
        for task in tasks {
            task.await.unwrap();
        }
    });

    // Drop the sender to close the channel
    drop(tx);

    // Process and print results from the receiver
    while let Ok((addr, port)) = rx.try_recv() {
        println!("= {addr}:{port}");
    }

    Ok(())
}

// Perform a scan on a specific address and port
async fn scan(
    addr: IpAddr,
    port: u16,
    results_tx: mpsc::Sender<(IpAddr, u16)>
) -> Result<(), mpsc::error::SendError<(IpAddr, u16)>> {
    // Try connecting to the address and port
    if let Ok(_ping) = TcpStream::connect((addr, port)).await {
        // If successful, send the result through the channel
        results_tx.send((addr, port)).await?;
    }

    Ok(())
}
