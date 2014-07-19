use std::io::net::tcp::TcpStream;
use std::str::from_utf8;

struct IRC {
  stream: TcpStream,
  connected: bool
}

impl IRC {
  fn connect() -> IRC {
    let mut irc: IRC;
    let mut stream = TcpStream::connect("irc.mozilla.org",6667).unwrap();
    stream.write("NICK homura-bot\n".as_bytes());
    stream.write("USER homura-bot 0 * :Best\n".as_bytes());
    stream.flush();
    let connected = false;
    return IRC { stream: stream, connected: connected };
  }
  fn read_line(&mut self) -> String {
    let mut buf = Vec::new();
    loop {
      let c = self.stream.read_byte().unwrap();
      if c == 0x0A {
        break;
      }
      buf.push(c);
    }
    let s = from_utf8(buf.as_slice()).unwrap();
    return s.to_string();
  }
  fn get_message(&mut self) -> String {
    loop {
      let line = self.read_line();
      let s = line.as_slice();
      println!("{}",s);
      if s.contains("PING") {
        println!("sending PONG");
        let split: Vec<&str> = s.as_slice().split(':').collect();
        self.stream.write("PONG ".as_bytes());
        self.stream.write(split[1].as_bytes());
        self.stream.write("\n".as_bytes());
        self.stream.flush();
      } else {
        return line.clone();
      }
    }
  }
  fn write(&mut self, s: &str) {
    self.stream.write(s.as_bytes());
  }
  fn flush(&mut self) {
    self.stream.flush();
  }
  fn msg(&mut self, channel: &str, msg: &str) {
    self.write("PRIVMSG ");
    self.write(channel);
    self.write(" :");
    self.write(msg);
    self.write("\n");
    self.flush();
  }
}

fn main() {
  let mut irc = IRC::connect();
  loop {
    let s = irc.get_message();
    if s.as_slice().contains(":homura-chan MODE") {
      println!("Registered!");
      irc.write("JOIN #interns\n");
    }
    if s.as_slice().contains("ben-せんぱい") {
      println!("Triggered");
      irc.msg("#interns","かわいいです");   
    }
  }
}
