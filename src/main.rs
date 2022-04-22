mod dm;
use std::collections::HashMap;
use std::{error, io, thread};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::process::id;
use std::ptr::null;
use std::sync::Mutex;
use std::thread::{sleep, Thread};
use std::time::Duration;
use lazy_static::lazy_static;
use reqwest;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::{Map, Option};
use dm::postbili::{dt, RunDate};

fn main() {
    let mut rundate = RunDate::get_instance("init.conf");
    thread::spawn(|| dt(&mut rundate) ).join();
}