use clap::{App, Arg, SubCommand};
use std::process;
use zook_network::{bridge, staking, governance, query}; // Import Zook modules

fn main() {
    let matches = App::new("Zook CLI")
        .version("1.0")
        .author("Zook Network")
        .about("Interact with the Zook network for bridging, staking, governance, and queries")
        .subcommand(
            SubCommand::with_name("bridge")
                .about("Manage BTCZ and zBTCZ conversions")
                .subcommand(
                    SubCommand::with_name("lock")
                        .about("Lock BTCZ to mint zBTCZ")
                        .arg(Arg::with_name("amount").required(true).help("Amount of BTCZ to lock")),
                )
                .subcommand(
                    SubCommand::with_name("unlock")
                        .about("Burn zBTCZ to release BTCZ")
                        .arg(Arg::with_name("amount").required(true).help("Amount of zBTCZ to burn")),
                )
                .subcommand(
                    SubCommand::with_name("status")
                        .about("Check bridge status and pending transactions"),
                ),
        )
        .subcommand(
            SubCommand::with_name("stake")
                .about("Manage zBTCZ staking")
                .subcommand(
                    SubCommand::with_name("stake")
                        .about("Stake zBTCZ to earn rewards")
                        .arg(Arg::with_name("amount").required(true).help("Amount of zBTCZ to stake")),
                )
                .subcommand(
                    SubCommand::with_name("unstake")
                        .about("Unstake zBTCZ")
                        .arg(Arg::with_name("amount").required(true).help("Amount of zBTCZ to unstake")),
                )
                .subcommand(
                    SubCommand::with_name("status")
                        .about("Check staking status"),
                ),
        )
        .subcommand(
            SubCommand::with_name("governance")
                .about("Participate in Zook governance")
                .subcommand(
                    SubCommand::with_name("vote")
                        .about("Vote on a governance proposal")
                        .arg(Arg::with_name("proposal-id").required(true).help("Proposal ID to vote on"))
                        .arg(Arg::with_name("vote").required(true).help("Vote 'yes' or 'no'")),
                )
                .subcommand(
                    SubCommand::with_name("propose")
                        .about("Create a governance proposal")
                        .arg(Arg::with_name("title").required(true).help("Title of the proposal"))
                        .arg(Arg::with_name("details").required(true).help("Details of the proposal")),
                )
                .subcommand(
                    SubCommand::with_name("status")
                        .about("Check current proposals and voting results"),
                ),
        )
        .subcommand(
            SubCommand::with_name("query")
                .about("Query network information")
                .subcommand(
                    SubCommand::with_name("balance")
                        .about("Check zBTCZ and gBTCZ balances"),
                )
                .subcommand(
                    SubCommand::with_name("rewards")
                        .about("Check staking rewards"),
                )
                .subcommand(
                    SubCommand::with_name("validators")
                        .about("List active validators and their statuses"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("bridge", Some(bridge_matches)) => match bridge_matches.subcommand() {
            ("lock", Some(lock_matches)) => {
                let amount = lock_matches.value_of("amount").unwrap();
                match bridge::lock(amount.parse().unwrap()) {
                    Ok(_) => println!("Successfully locked {} BTCZ.", amount),
                    Err(e) => println!("Error locking BTCZ: {}", e),
                }
            }
            ("unlock", Some(unlock_matches)) => {
                let amount = unlock_matches.value_of("amount").unwrap();
                match bridge::unlock(amount.parse().unwrap()) {
                    Ok(_) => println!("Successfully unlocked {} BTCZ.", amount),
                    Err(e) => println!("Error unlocking BTCZ: {}", e),
                }
            }
            ("status", _) => {
                match bridge::status() {
                    Ok(status) => println!("Bridge Status: {:?}", status),
                    Err(e) => println!("Error fetching bridge status: {}", e),
                }
            }
            _ => println!("Invalid bridge command"),
        },
        ("stake", Some(stake_matches)) => match stake_matches.subcommand() {
            ("stake", Some(stake_args)) => {
                let amount = stake_args.value_of("amount").unwrap();
                match staking::stake(amount.parse().unwrap()) {
                    Ok(_) => println!("Successfully staked {} zBTCZ.", amount),
                    Err(e) => println!("Error staking zBTCZ: {}", e),
                }
            }
            ("unstake", Some(unstake_args)) => {
                let amount = unstake_args.value_of("amount").unwrap();
                match staking::unstake(amount.parse().unwrap()) {
                    Ok(_) => println!("Successfully unstaked {} zBTCZ.", amount),
                    Err(e) => println!("Error unstaking zBTCZ: {}", e),
                }
            }
            ("status", _) => {
                match staking::status() {
                    Ok(status) => println!("Staking Status: {:?}", status),
                    Err(e) => println!("Error fetching staking status: {}", e),
                }
            }
            _ => println!("Invalid staking command"),
        },
        ("governance", Some(governance_matches)) => match governance_matches.subcommand() {
            ("vote", Some(vote_matches)) => {
                let proposal_id = vote_matches.value_of("proposal-id").unwrap();
                let vote = vote_matches.value_of("vote").unwrap();
                match governance::vote(proposal_id.parse().unwrap(), vote) {
                    Ok(_) => println!("Successfully voted '{}' on proposal {}.", vote, proposal_id),
                    Err(e) => println!("Error voting on proposal: {}", e),
                }
            }
            ("propose", Some(propose_matches)) => {
                let title = propose_matches.value_of("title").unwrap();
                let details = propose_matches.value_of("details").unwrap();
                match governance::propose(title, details) {
                    Ok(_) => println!("Proposal '{}' created successfully.", title),
                    Err(e) => println!("Error creating proposal: {}", e),
                }
            }
            ("status", _) => {
                match governance::status() {
                    Ok(status) => println!("Governance Status: {:?}", status),
                    Err(e) => println!("Error fetching governance status: {}", e),
                }
            }
            _ => println!("Invalid governance command"),
        },
        ("query", Some(query_matches)) => match query_matches.subcommand() {
            ("balance", _) => {
                match query::balance() {
                    Ok(balance) => println!("Balances: {:?}", balance),
                    Err(e) => println!("Error fetching balances: {}", e),
                }
            }
            ("rewards", _) => {
                match query::rewards() {
                    Ok(rewards) => println!("Rewards: {:?}", rewards),
                    Err(e) => println!("Error fetching rewards: {}", e),
                }
            }
            ("validators", _) => {
                match query::validators() {
                    Ok(validators) => println!("Validators: {:?}", validators),
                    Err(e) => println!("Error fetching validators: {}", e),
                }
            }
            _ => println!("Invalid query command"),
        },
        _ => {
            println!("Invalid command. Use --help for usage information.");
            process::exit(1);
        }
    }
}
