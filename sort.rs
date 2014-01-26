/* Sorting experiments */
extern mod extra;
use std::vec;
use std::rand;
use std::rand::distributions::{IndependentSample, Range};
use extra::time::precise_time_ns;

fn insertionsort(list: &mut [int]) {
	for j in range(1, list.len() as int) {
		let key = list[j];
		let mut i = j - 1;
		while i >= 0 && list[i] > key {
			list[i + 1] = list[i];
			i -= 1;
		}
		list[i + 1] = key;
	}
}

fn merge(list: &mut [int], p: uint, q: uint, r: uint) {
	let n1 = q - p + 1;
	let n2 = r - q;
	let mut left: ~[int];
	let mut right: ~[int];
	left = vec::from_elem(n1 + 1, 0);
	right = vec::from_elem(n2 + 1, 0);
	for i in range(0, n1) {
		left[i] = list[p + i];
	}
	for j in range(0, n2) {
		right[j] = list[q + j + 1];
	}
	left[n1] = std::int::max_value;
	right[n2] = std::int::max_value;
	let mut i = 0;
	let mut j = 0;
	for k in range(p, r + 1) {
		if left[i] <= right[j] {
			list[k] = left[i];
			i += 1;
		} else {
			list[k] = right[j];
			j += 1;
		}
	}
}

fn mergesorter(list: &mut [int], p: uint, r: uint) {
	if p < r {
		let q = (p + r) / 2;
		mergesorter(list, p, q);
		mergesorter(list, q + 1, r);
		merge(list, p, q, r);
	}
}

/* why do I have to save list length in order to use as an arg? */
fn mergesort(list: &mut [int]) {
	let length = list.len();
	mergesorter(list, 0, length - 1);
}

fn main() {
	/* XXX put these next two together in a struct or something */
	let sorternames = ["insertionsort", "mergesort"];
	let sorters = [insertionsort, mergesort];
	let mut list: ~[int];

	list = vec::with_capacity(1000000);
	let between = Range::new(-100000000, 100000000);
	let mut rng = rand::task_rng();
	let begin = precise_time_ns();
	for _ in range(0,50000) {
		list.push(between.ind_sample(&mut rng));
	}
	let elapsed = precise_time_ns() - begin;
	println!("Generating list of {} took {} ms", list.len(), elapsed/1000000);

	for a in range(0, sorters.len()) {
		let mut listtosort = list.to_owned();
		let begin = precise_time_ns();
		(sorters[a])(listtosort);
		let elapsed = precise_time_ns() - begin;
		println!("{:?} took {} ms", sorternames[a], elapsed/1000000);
		for i in range(0, listtosort.len() - 1) {
			if (listtosort[i] > listtosort[i + 1]) {
				println!("{:?} has an error at entry {}", sorternames[a], i);
				break;
			}
		}
	}
}

