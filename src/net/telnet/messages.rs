/// Enumeration of the different negotiations that can be performed for a Telnet Option
pub enum Negotiation {
    Do,
    Dont,
    Will,
    Wont
}

/// Enumeration of the different commands that can be represented as Telnet messages
pub enum Command {
    DataMark,
    Break,
    InterruptProcess,
    AbortOutput,
    AreYouThere,
    EraseCharacter,
    EraseLine,
    GoAhead
}

/// Enumeration of the different Telnet Messages that can be represented
pub enum TelnetMessage {
    /// Telnet Message representing a single bare byte
    /// # Example
    /// ```
    /// # use libelinvar::net::telnet::*;
    /// TelnetMessage::Byte(200);
    /// ```
    Byte(u8),

    /// Telnet Message representing a protocol-level command
    /// # Example
    /// ```
    /// # use libelinvar::net::telnet::*;
    /// TelnetMessage::Command(Command::AbortOutput);
    /// ```
    Command(Command),

    /// Telnet Message representing an option negotiation
    /// # Example
    /// ```
    /// # use libelinvar::net::telnet::*;
    /// TelnetMessage::Negotiation(Negotiation::Do, 31);
    /// ```
    Negotiation(Negotiation, u8),

    /// Telnet Message representing an option subnegotiation
    /// # Example
    /// ```
    /// # use libelinvar::net::telnet::*;
    /// TelnetMessage::SubNegotiation(31, vec!(0, 80, 0, 24));
    /// ```
    SubNegotiation(u8, Vec<u8>)
}
