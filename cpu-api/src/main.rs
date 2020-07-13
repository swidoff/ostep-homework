use std::env;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;


use nix::fcntl::{OFlag, open};
use nix::sys::stat::Mode;
use nix::sys::wait::{waitpid, wait};
use nix::unistd::{fork, ForkResult, write, execv, close, pipe, read};
use std::ffi::{CString, CStr};
use std::io::stdout;
use std::os::unix::io::AsRawFd;


fn main() {
    let args = env::args();
    if args.len() != 2 {
        eprintln!("Expected a single argument ");
        exit(1);
    }

    match args.skip(1).next().unwrap().parse::<u8>() {
        Ok(1) => program1(),
        Ok(2) => program2(),
        Ok(3) => program3(),
        Ok(4) => program4(),
        Ok(5) => program5(),
        Ok(6) => program6(),
        Ok(7) => program7(),
        Ok(8) => program8(),
        Ok(num) => eprintln!("Unknown program number: {}", num),
        Err(_) => eprintln!("Expected a numeric argument."),
    }
}

fn program1() {
    let mut x = 100;
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Parent: Child has pid: {}, x: {}", child, x);
            x = 10;
            println!("Parent: x: {}", x);
        }
        Ok(ForkResult::Child) => {
            println!("Child: x: {}", x);
            x = 20;
            println!("Child: x: {}", x);
        }
        Err(_) => println!("Fork failed"),
    }
}

fn program2() {
    let fd = open("program2.out", OFlag::O_RDWR | OFlag::O_CREAT, Mode::S_IWUSR | Mode::S_IRUSR)
        .expect("Unable to open file");

    match fork() {
        Ok(ForkResult::Parent { child: _, .. }) => {
            for _ in 1..10 {
                match write(fd, b"Parent: A bunch of bytes\n") {
                    Ok(b) => println!("Parent: wrote {} bytes", b),
                    Err(_) => println!("Parent: Failed to write"),
                };
                sleep(Duration::from_secs(1));
            }
        }
        Ok(ForkResult::Child) => {
            for _ in 1..10 {
                match write(fd, b"Child: Another bunch of bytes\n") {
                    Ok(b) => println!("Child: wrote {} bytes", b),
                    Err(_) => println!("Child: Failed to write"),
                }
                sleep(Duration::from_secs(1))
            }
        }
        Err(_) => println!("Fork failed"),
    }
}

fn program3() {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).expect("Wait failed.");
            println!("Goodbye")
        }
        Ok(ForkResult::Child) => {
            println!("Hello")
        }
        Err(_) => println!("Fork failed"),
    }
}

fn program4() {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Parent: Child pid is {}", child)
        }
        Ok(ForkResult::Child) => {
            let current_dir = CString::new(".").expect("CString::new failed");
            let argv: [&CStr; 1] = [&current_dir];
            let command = CString::new("/bin/ls").expect("CString::new failed");
            execv(&command, &argv).expect("execv failed");
        }
        Err(_) => println!("Fork failed"),
    }
}

fn program5() {
    match fork() {
        Ok(ForkResult::Parent { child: _, .. }) => {
            let res = wait().expect("Wait failed.");
            println!("Goodbye");
            println!("{:?}", res);
        }
        Ok(ForkResult::Child) => {
            // let res = wait().expect("Wait failed.");
            println!("Hello");
            // println!("{:?}", res);
        }
        Err(_) => println!("Fork failed"),
    }
}

fn program6() {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            let res = waitpid(child, None).expect("Waitpid failed.");
            println!("Goodbye");
            println!("{:?}", res);
        }
        Ok(ForkResult::Child) => {
            // let res = wait().expect("Wait failed.");
            println!("Hello");
            // println!("{:?}", res);
        }
        Err(_) => println!("Fork failed"),
    }
}

fn program7() {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Parent: Child has pid: {}", child);
        }
        Ok(ForkResult::Child) => {
            let stdout_fd = stdout().as_raw_fd();
            println!("Child: stdout fd is: {}", stdout_fd);
            close(stdout_fd).expect("Unable to close stdout");
            println!("Child: writing after stdout has been closed.");
        }
        Err(_) => println!("Fork failed"),
    }
}

fn program8() {
    let (pipe_read, pipe_write) = pipe().expect("Failed to create pipe.");

    match fork() {
        Ok(ForkResult::Parent { child: _, .. }) => {
            close(pipe_read).expect("Close pipe_read failed");
            write(pipe_write, b"A message from the parent: Behave, my child!").expect("Write failed");
            close(pipe_write).expect("Close pipe_write failed");
            exit(0);
        },
        Ok(ForkResult::Child) => {
            let mut buf: [u8; 256] = [0; 256];
            close(pipe_write).expect("Close pipe_write failed");
            if let Ok(size) = read(pipe_read, &mut buf) {
                println!("Child: {}", String::from_utf8_lossy(&buf[0..size]))
            } else {
                eprintln!("Child: failed to read from buf.")
            }
            close(pipe_read).expect("Close pipe_read failed");
        },
        Err(_) => println!("Fork failed"),
    }
}