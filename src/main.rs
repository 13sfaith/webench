use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{self, BufRead};
use std::time::SystemTime;
use std::time::Duration;
use std::env;

struct Response {
	code: String,
	success: bool,
	time: u128,
	bytes: u64,
}// A data type to store the info from each response

fn main() -> std::io::Result<()>{ 
	// pull the arguments from the command line
	let args: Vec<String> = env::args().collect();

	// http vars
    let mut param = "/".to_string();
    let mut url = String::new();

    // profiling vars
    let mut num_req = 1;
	let mut profiling = false;

	let mut help = false;

    // handle arguments and assign proper vars/state
    for i in 0..args.len()
    {
        if args.get(i).unwrap_or(&"--help".to_string()) == "--help"
        {
            help = true;
        }
        else if args.get(i).unwrap_or(&"-".to_string()) == "--url" && num_req != 0
        {
            let mut is_param = false;
            for c in args.get(i+1).unwrap().chars()
            {
                if !is_param && c == '/'
                {
                    is_param = true;
                }
                else if is_param
                {
    				param.push(c);
                } else {
                    url.push(c);
                }
            }
        }
        else if args.get(i).unwrap() == "--profile" && num_req != 0
        {
            num_req = args.get(i+1).unwrap().parse().unwrap_or(0);
			profiling = true;
        }
    }

	if help
	{
		print_help();
	}
	else if profiling
	{
		let stats = handle_profile(url, param, num_req);
		handle_stats(stats);
	}
	else
	{
		handle_net(url, param);
	}

    Ok(())
}

fn print_help()
{
	println!("\n\nbasic HTTP request/benchmark tool");
	println!("\nUSAGE:");
	println!("\twebench [COMMAND] [OPTIONS]");
	println!("\nCOMMAND:");
	println!("\t--url <domain>\tUrl flag (ex. ");
	println!("\t--help\tHelp flag");
	println!("\nOPTIONS:");
	println!("\t--profile <+int>\tNumber of times to test the url\n");


}

// input - url (ex. general.13sfaith.com), param (ex. /links), num_of_req (ex. 5)
// sends a request to the domain num_of_req number of times and load information into a list
// output - null
fn handle_profile(url: String, param: String, num_of_req: i32) -> Vec<Response> 
{
	let mut stats: Vec<Response> = Vec::new(); // vector to store all the res data

	for _i in 0..num_of_req
	{
		// Establish tcp connection
    	let stream = TcpStream::connect(format!("{}:80", url)).unwrap();
		stream.set_read_timeout(Some(Duration::new(1,0))).unwrap();

		// Create HTTP request
		let header = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", param, url);

		// Start timer
		let now = SystemTime::now();

		// Send request and put response in buffer and printout
		let buffer = send_req(stream, header);


		// temporary data for creating the struct
		let mut t_code: String = "".to_string(); 
		let mut t_succ: bool = false;

		let mut data_size: u64 = 0;

		let mut t_time: u128 = 0;


		for line in buffer.lines()
		{
			let cur = line.unwrap_or("]".to_string()); // If the timeout is called insert end key ']'
			data_size += (cur.len() * 4) as u64;

			if cur.contains("HTTP")// finding the status of the response
			{
				t_time = now.elapsed().unwrap().as_millis();

				let arr: Vec<char> = cur.chars().collect();
				if arr[9] == '2'
				{
					t_succ = true;
				}
				t_code = String::from(arr[9].to_string() + &arr[10].to_string() + &arr[11].to_string());
				//t_code.push(arr[10].to_string());
				//t_code.push(arr[11].to_string());
			}
			if cur == "</html>" || cur == "]" // end cases
			{
				break;
			}
		}
		
		stats.push(Response {
						code: t_code,
						success: t_succ,
						time: t_time,
						bytes: data_size
					});
	}

	stats // after all data collected return the stats vector
}


// input - url (ex. cloudflare.com), param (ex. /workers)
// sends a request to the server and prints its response line by line to terminal
// output - null
fn handle_net(url: String, param: String) {

	// Establish tcp connection
	let stream = TcpStream::connect(format!("{}:80", url)).unwrap();
	stream.set_read_timeout(Some(Duration::new(1,0))).unwrap(); // handles buffer staying open issues

	// Create HTTP request
	let header = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", param, url);

	// Send request and put response in buffer and printout
	let buffer = send_req(stream, header);

	for line in buffer.lines()
	{
		let cur = line.unwrap_or("]".to_string()); // if timeout put emergency stop - ']'
		println!("{}", cur);
		if cur.contains("</html>") || cur == "]"
		{
			break;
		}
	}
}

// input: a list of Response structs
// prints an organized set of data
// output: null
fn handle_stats(mut stats: Vec<Response>){
	
	// sort the vector from low to high (response time)
	stats.sort_by(|a, b| b.time.cmp(&a.time));

	// stat variables
	let length = stats.len();

	let fast = stats[length - 1].time;
	let slow = stats[0].time;	

	let mut large_byte = 0;
	let mut small_byte = 1000000;

	let mut sum_times = 0;

	let median;
	if length % 2 == 0 {median = (stats[(length-1) / 2].time + stats[((length-1) / 2) + 1].time) / 2; }
	else {median = stats[(length-1) / 2].time; }

	let mut stat_sum = 0;
	let mut suc_sum = 0;

	// creating a hashmap of possible error codes
	let mut stat_codes = HashMap::new();

	println!("# of requests: {}", length);
	for element in stats
	{
		sum_times += element.time;
		if large_byte < element.bytes { large_byte = element.bytes; }
		if small_byte > element.bytes { small_byte = element.bytes; }

		stat_sum += 1;
		if element.success
		{
			suc_sum += 1;
		// if the response is not a success either add or append to the hashmap
		} else {
			if stat_codes.contains_key(&element.code) 
			{
				let tmp_val = stat_codes[&element.code];
				stat_codes.insert(
						element.code,
						tmp_val + 1,
				);
			} else {
				stat_codes.insert(
						element.code,
						1,
				);
			}
		}
			
	}

	let suc_rate = (suc_sum / stat_sum) * 100;
	
	println!("Fastest time: {}ms", fast);
	println!("Slowest time: {}ms", slow);
	println!("Mean time: {}ms", sum_times / length as u128);
	println!("Median time: {}ms", median);
	println!("Success requests: {}%", suc_rate);
	if suc_rate < 100
	{
		println!("Error Codes: ");
		for (key, val) in stat_codes.iter()
		{
			println!("\t{} {}", key, val);
		}
	}
	println!("Smallest Response: {}b", small_byte);
	println!("Largest Response: {}b", large_byte);
}

// Input - connect: TcpStream initalized to the url, Header: String with the HTTP request
// Write the request to the TcpStream, return a buffer with the response
// Output - BufReader: buffer containing the response from the server 
fn send_req(mut connect: TcpStream, header: String) -> io::BufReader<TcpStream>{
    connect.write(header.as_bytes()).unwrap();    
    connect.flush().unwrap();

    io::BufReader::new(connect) 
}
