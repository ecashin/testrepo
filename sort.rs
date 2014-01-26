/* Sorting experiments */
extern mod extra;
use std::vec;
use std::rand;
use std::rand::distributions::{IndependentSample, Range};
use extra::time::precise_time_ns;

enum Globals {
	DistMax = 10000000,
	ListSize = 6000000,
}

/* Insertion Sort */
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

/* Merge Sort */
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

/* Heap Sort */
fn heapleft(i: int) -> int {
	2*i + 1
}

fn heapright(i: int) -> int {
	2*i + 2
}

fn maxheapify(list: &mut [int], i: int, size: uint) {
	let mut largest;
	let l = heapleft(i);
	let r = heapright(i);
	if (l as uint) < size && list[l] > list[i] {
		largest = l;
	} else {
		largest = i;
	}
	if (r as uint) < size && list[r] > list[largest] {
		largest = r;
	}
	if largest != i {
		let tmp = list[i];
		list[i] = list[largest];
		list[largest] = tmp;
		maxheapify(list, largest, size);
	}
}

fn buildmaxheap(list: &mut [int]) {
	let size = list.len();
	/* this next line is broken?--rust bug? */
	//for i in std::iter::range_step(list.len()/2 - 1, -1, -1) {
	let mut i: int = list.len() as int /2 - 1;
	while (i >= 0) {
		maxheapify(list, i, size);
		i -= 1;
	}
}

fn heapsort(list: &mut [int]) {
	buildmaxheap(list);
	let mut size = list.len();
	/* this next line is broken--rust bug?! */
	//for i in std::iter::range_step(list.len() - 1, 0, -1) {
	let mut i: int = list.len() as int - 1;
	while (i > 0) {
		let tmp = list[0];
		list[0] = list[i];
		list[i] = tmp;
		size -= 1;
		maxheapify(list, 0, size);
		i -= 1;
	}
}

/* quicksort */
fn quicksort(list: &mut[int]) {
	let length = list.len() as int;
	quicksorter(list, 0, length - 1);
}

fn quicksorter(list: &mut[int], p: int, r: int) {
	if p < r {
		let q = partition(list, p, r);
		quicksorter(list, p, q - 1);
		quicksorter(list, q + 1, r);
	}
}

fn partition(list: &mut[int], p: int, r: int) -> int {
	let x = list[r];
	let mut i = p - 1;
	let mut j = p;
	while (j < r) {
		if list[j] <= x {
			i += 1;
			let tmp = list[i];
			list[i] = list[j];
			list[j] = tmp;
		}
		j += 1;
	}
	let tmp = list[i + 1];
	list[i + 1] = list[r];
	list[r] = tmp;
	return i + 1;
}

/* randomized quicksort */
fn randquicksort(list: &mut[int]) {
	let length = list.len() as int;
	let mut rng: rand::XorShiftRng = rand::weak_rng();
	randquicksorter(list, 0, length - 1, &mut rng);
}

fn randquicksorter(list: &mut[int], p: int, r: int, rng: &mut rand::XorShiftRng) {
	if p < r {
		let q = randpartition(list, p, r, rng);
		randquicksorter(list, p, q - 1, rng);
		randquicksorter(list, q + 1, r, rng);
	}
}

fn randpartition(list: &mut[int], p: int, r: int, rng: &mut rand::XorShiftRng) -> int {
	let between = Range::new(p, r);
	let i = between.ind_sample(rng) as int;
	let tmp = list[r];
	list[r] = list[i];
	list[i] = tmp;
	return partition(list, p, r);
}

/* counting sort */
fn countingsort(list: &mut [int]) {
	let max = DistMax as uint;
	let mut work: ~[int];
	let mut copy: ~[int];
	work = vec::from_elem(max + 1, 0);
	copy = list.to_owned();
	for j in copy.iter() {
		work[*j] += 1;
	}
	for i in range(1, max + 1) {
		work[i] += work[i - 1];
	}
	let mut j = copy.len() as int - 1;
	while (j >= 0) {
		list[work[copy[j]] - 1] = copy[j];
		work[copy[j]] -= 1;
		j -= 1;
	}
}

/* radix sort */
fn countingsortrad(list: &mut [int], digit: uint) {
	let pow10 = [1, 10, 100, 1000, 10000, 100000, 1000000,
		10000000, 100000000, 1000000000];
	let max = 9;
	let mut work: ~[int];
	let mut copy: ~[int];
	work = vec::from_elem(max + 1, 0);
	copy = list.to_owned();
	for j in copy.iter() {
		let idx = (*j / pow10[digit]) % 10;
		work[idx] += 1;
	}
	for i in range(1, max + 1) {
		work[i] += work[i - 1];
	}
	let mut j = copy.len() as int - 1;
	while (j >= 0) {
		let idx = (copy[j] / pow10[digit]) % 10;
		list[work[idx] - 1] = copy[j];
		work[idx] -= 1;
		j -= 1;
	}
}

fn radixsort(list: &mut [int]) {
	let mut x = DistMax as uint;
	let mut digits = 0;
	while (x > 0) {
		x /= 10;
		digits += 1;
	}
	for i in range(0, digits) {
		countingsortrad(list, i as uint);
	}
}

fn main() {
	/* XXX put these next two together in a struct or something */
	let sorternames = [
		//"insertionsort", 
		"mergesort", 
		"heapsort",
		"quicksort",
		"randquicksort",
		"countingsort",
		"radixsort",
	];
	let sorters = [
		//insertionsort, 
		mergesort, 
		heapsort,
		quicksort,
		randquicksort,
		countingsort,
		radixsort,
	];
	let mut list: ~[int];

	list = vec::with_capacity(ListSize as uint);
	let between = Range::new(0, DistMax as int);
	let mut rng = rand::rng();
	let begin = precise_time_ns();
	for _ in range(0,ListSize as uint) {
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
				println!("{:?} has an error at index {}", sorternames[a], i);
				if (listtosort.len() < 100) {
					for x in listtosort.iter() {
						print!("{} ", *x);
					}
				}
				print("\n");
				break;
			}
		}
	}
}

