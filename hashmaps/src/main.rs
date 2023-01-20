use std::io;

fn main() {
	
	// Listing 8-20: Creating a new hash map and inserting some keys and values
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
	
	// Accessing Values in a Hash Map
	let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None. This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key.
	
	for (key, value) in &scores {
		
		println!("{}: {}", key, value);
	}
	
	// Updating a Hash Map
	// If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced    
	// Listing 8-23: Replacing a value stored with a particular key
	scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
	
	// Adding a Key and Value Only If a Key Isn’t Present
	// if the key does exist in the hash map, the existing value should remain the way it is. If the key doesn’t exist, insert it and a value for it.
	
	// Listing 8-24: Using the entry method to only insert if the key does not already have a value
	let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
	
	// Updating a Value Based on the Old Value
	// Listing 8-25: Counting occurrences of words using a hash map that stores words and counts
	let text = "hello world wonderful world";

    let mut map = HashMap::new();
	
	// We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0.
    for word in text.split_whitespace() { // The split_whitespace method returns an iterator over sub-slices, separated by whitespace, of the value in text
        let count = map.entry(word).or_insert(0); // The or_insert method returns a mutable reference (&mut V) to the value for the specified key
        *count += 1;
    }

    println!("{:?}", map);
	
	// some exercises you should now be equipped to solve:

    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
	
	let mut v = vec![15, 1, 90, 35, 30, 4, 70, 70];   
	let mut sum = 0;
	
	for i in &mut v {
        sum += *i;
    };
	
	println!("{}", sum);
	
	let mut mean:f32 = sum as f32 / v.len() as f32;
	
	//mean
	println!("mean = {}", mean);
	
	//median	
	
	v.sort();
	println!("sorted vector = {v:?}");
	
	let middle_index = v.len()/2;
	
	println!("middle_index = {}", middle_index);
	
	if v.len() % 2 == 0 {
		v.sort();
		let mut median:f32 = (*&v[(middle_index-1)] as f32+*&v[(middle_index)] as f32)/2.0;		
		println!("median (even) = {}", median);
	}	
	else {
		v.sort();
		let mut median = v[middle_index];
		println!("median (odd) = {}", median);
	}	
	
	//mode
    let mut map = HashMap::new();
	
    for i in &mut v { 
        let count = map.entry(*i).or_insert(0); 
        *count += 1;
    }	

	let mut mode = 0;
	let mut max_count = 0;
	
	for (number, c) in map {
		if c > max_count {
			max_count = c;
			mode = number;
		}
	}	
	
	println!("mode = {}", mode);
	
	/* 	
	// from
	// https://benjaminbrandt.com/averages-in-rust/
	
	fn mean(list: &[i32]) -> f64 {
		let sum: i32 = Iterator::sum(list.iter());
		f64::from(sum) / (list.len() as f64)
	}
	
	fn median(list: &[i32]) -> f64 {
		let len = list.len();
		let mid = len / 2;
		if len % 2 == 0 {
			mean(&list[(mid - 1)..(mid + 1)])
		} else {
			f64::from(list[mid])
		}
	}
	
	
	fn mode(list: &[i32]) -> i32 {
		let mut occurrences: HashMap<&i32, i32> = HashMap::new();
		let mut max: (i32, i32) = (0, 0);

		for entry in list {
			let count = occurrences.entry(entry).or_insert(0);
			*count += 1;
		}

		for (&&key, &val) in &occurrences {
			if val > max.1 {
				max = (key, val);
			}
		}

		max.0
	} 
	*/
	
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
	
	

	// https://gist.github.com/TheCreatorAMA/ad3254a96ed2605eb64947d1fdc338ee

	fn convert_to_pig_latin(input_str: &String) -> String {
		let mut str_array = vec![];

		// Splitting up words in input string
		for word in input_str.to_lowercase().split_whitespace() {

			// Init prefix and ending variables
			let mut prefix = String::from("");
			let mut ending = String::from("");

			let mut count = 0;

			// Loop through chars of each word. Counter is used to tell whether or not we are at the
			// first char of a word, if so check if it is a vowel or not. If not on first char add
			// chars to prefix. 
			for char in word.chars() {
				match char {
					'a' | 'e' | 'i' | 'o' | 'u' => {
						if count == 0 {
							ending.push_str("hay");
							prefix.push(char);
						} else {
							prefix.push(char);
						}
					}
					_ => {
						if count == 0 {
							ending.push_str(char.to_string().as_str());
							ending.push_str("ay");
						} else {
							prefix.push(char);
						}
					}
				}
				count += 1;
			}
			// Join prefix and ending together to form new "pig latin" word. Push this new word to the
			// array of strings
			str_array.push(prefix + &"-".to_string() + &ending);
		}
		
		// Join array of pig latin strings together and return
		str_array.join(" ")
	}

	let normal_str = String::from("hello wonderful world");
    let pig_latin = convert_to_pig_latin(&normal_str);

    println!("Input: {}\nOutput: {}", normal_str, pig_latin);

	// https://gist.github.com/DenialAdams/06d6a8c841c7e328ce986a299f786ead
	fn piglatinize(s: &mut String) {
    let initial_letter = s.remove(0);
    if !is_vowel(initial_letter) {
        s.push(initial_letter);
        s.push_str("-ay");
    } else {
        s.push_str("-hay");
    }
	}

	fn is_vowel(c: char) -> bool {
		c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
	}
	
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        piglatinize(&mut input);
        println!("{}", input);
        input.clear();
    }



	// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
	
	// add_employees\src\main.rs

}

















































