extern crate clap;
use clap::{crate_authors, crate_version, ArgAction, Parser};

/*
Originally from: https://github.com/jakewilliami/scripts/blob/master/bash/local-net

Usage: local-net [option...]

The present script will ping local machines using different tools (see options).  Also see the command `cstatus`.

         -a | --arp                Prints arp results.
         -A | --minimal-arp        Prints a Minimal arp output.
         -g | --geo-location       Shows geo-location information based on one's (public) IP address.
         -H | --Hostnames          [OPTION IN DEVELOPMENT] Finds Hostnames within the local network.
         -l | --local              Returns local IP address of present computer.
         -m | --malware            Check the present computer for malware (in the form of rootkits).
         -n | --nmap               Prints nmap results.
         -p | --ping               Prints pinged results.
         -P | --Public             Returns Public IP address of the present local network.
         -q | --query-server       Query servers on the local network.
         -s | --sniff              Sniffs packages exchanged through the local network.
         -S | --sniff-alt          Sniffs using alternate method.
         -v | --verbose-sniff      Verbosely sniffs packages exchanged through the local network.
         -V | --verbose-sniff-alt  Verbosely sniffs using alternate methods
         -x | --arp-poisoning      DO NOT USE. Begins ARP-poisoning the local network.
         -h | --help               Shows help (present output)
*/
#[derive(Parser)]
#[command(
    name = "teanet",
    author = crate_authors!("\n"),
    version = crate_version!(),
    about = "A command line tool for querying information about your home network",
)]
struct Cli {
    /// Display
    #[arg(
        short = 'l',
        long = "local",
        short_alias = 'p',
        alias = "private",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    local_ip: Option<bool>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(show_local_ip) = cli.local_ip {
        if show_local_ip {
            todo!();
        }
    }
}
