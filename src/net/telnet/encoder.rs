use super::{TelnetMessage, Command, Negotiation, bytes};

/// Encode a given Telnet Message into the bytes that represent it on the wire.
///
/// # Parameters
/// * msg - The message to be encoded
///
/// # Returns
/// A vector of bytes representing the provided Telnet Message. These are the exact bytes that
/// can be used on the Telnet network connection
///
/// # Example
/// ```
/// # use libelinvar::net::telnet::*;
/// let message = TelnetMessage::Negotiation(Negotiation::Do, 31);
/// let encoded = encode_telnet_message(message);
/// assert_eq!(encoded, vec!(bytes::IAC, bytes::DO, 31));
/// ```
pub fn encode_telnet_message(msg: TelnetMessage) -> Vec<u8> {
    match msg {
        TelnetMessage::Byte(b) => encode_byte_message(b),
        TelnetMessage::Command(c) => encode_command_message(c),
        TelnetMessage::Negotiation(n, o) => encode_negotiation_message(n, o),
        TelnetMessage::SubNegotiation(o, p) => encode_subnegotiation_message(o, p)
    }
}

/// Encode a Byte Message into the appropriate bytes.
/// If the byte is an IAC then it needs to be escaped. Otherwise it can be used as-is
///
/// # Parameters
/// * msg - The actual byte to be encoded
///
/// # Returns
/// The encoded bytes that represent this message.
fn encode_byte_message(msg: u8) -> Vec<u8> {
    if msg == bytes::IAC {
        vec!(bytes::IAC, bytes::IAC)
    } else {
        vec!(msg)
    }
}

/// Encode a Command Message into the appropriate bytes.
///
/// # Parameters
/// * command - The actual telnet command to be encoded
///
/// # Returns
/// The encoded bytes that represent this message.
fn encode_command_message(command: Command) -> Vec<u8> {
    let command_byte = match command {
        Command::DataMark => bytes::DATA_MARK,
        Command::Break => bytes::BREAK,
        Command::InterruptProcess => bytes::INTERRUPT_PROCESS,
        Command::AbortOutput => bytes::ABORT_OUTPUT,
        Command::AreYouThere => bytes::ARE_YOU_THERE,
        Command::EraseCharacter => bytes::ERASE_CHARACTER,
        Command::EraseLine => bytes::ERASE_LINE,
        Command::GoAhead => bytes::GO_AHEAD
    };

    vec!(bytes::IAC, command_byte)
}

/// Encode a Negotiation Message into the appropriate bytes.
///
/// # Parameters
/// * negotiation - The actual Telnet negotiation to be encoded
/// * option - The ID of the option that is being negotiated
///
/// # Returns
/// The encoded bytes that represent this message.
fn encode_negotiation_message(negotiation: Negotiation, option: u8) -> Vec<u8> {
    let negotiation_byte = match negotiation {
        Negotiation::Do => bytes::DO,
        Negotiation::Dont => bytes::DONT,
        Negotiation::Will => bytes::WILL,
        Negotiation::Wont => bytes::WONT
    };

    vec!(bytes::IAC, negotiation_byte, option)
}

/// Encode a SubNegotiation Message into the appropriate bytes.
///
/// # Parameters
/// * option - The ID of the option that is being negotiated
/// * payload - The payload of the option negotiation
///
/// # Returns
/// The encoded bytes that represent this message.
fn encode_subnegotiation_message(option: u8, payload: Vec<u8>) -> Vec<u8> {
    let mut message = vec!(bytes::IAC, bytes::SB, option);

    // TODO: I'm convinced this can be done using flat_map, but can't yet work out how
    for payload_byte in payload {
        for escaped_byte in encode_byte_message(payload_byte) {
            message.push(escaped_byte);
        }
    }

    // TODO: I'm sure these can be done as one call
    message.push(bytes::IAC);
    message.push(bytes::SE);

    message
}

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn test_encode_byte() {
        let message = TelnetMessage::Byte(200);
        let encoded = encode_telnet_message(message);
        assert_eq!(vec!(200), encoded);
    }

    #[test]
    fn test_encode_iac_byte() {
        let message = TelnetMessage::Byte(255);
        let encoded = encode_telnet_message(message);
        assert_eq!(vec!(255, 255), encoded);
    }

    #[test]
    fn test_encode_abort_output() {
        let message = TelnetMessage::Command(Command::AbortOutput);
        let encoded = encode_telnet_message(message);
        assert_eq!(vec!(255, 245), encoded);
    }
    #[test]
    fn test_encode_negotiation() {
        let message = TelnetMessage::Negotiation(Negotiation::Wont, 31);
        let encoded = encode_telnet_message(message);
        assert_eq!(vec!(255, 252, 31), encoded);
    }

    #[test]
    fn test_encode_empty_subnegotiation() {
        let message = TelnetMessage::SubNegotiation(31, vec!());
        let encoded = encode_telnet_message(message);
        assert_eq!(vec!(255, 250, 31, 255, 240), encoded);
    }

    #[test]
    fn test_encode_simple_subnegotiation() {
        let message = TelnetMessage::SubNegotiation(31, vec!(0, 80, 0, 24));
        let encoded = encode_telnet_message(message);
        assert_eq!(vec!(255, 250, 31, 0, 80, 0, 24, 255, 240), encoded);
    }

    #[test]
    fn test_encode_subnegotiation_containing_iac() {
        let message = TelnetMessage::SubNegotiation(31, vec!(0, 255, 0, 24));
        let encoded = encode_telnet_message(message);
        assert_eq!(vec!(255, 250, 31, 0, 255, 255, 0, 24, 255, 240), encoded);
    }
}
