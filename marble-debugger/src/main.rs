use std::{
    fs,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use dap::{
    events::{ExitedEventBody, OutputEventBody},
    prelude::*,
    types::{Capabilities, OutputEventCategory},
};
use marble::source::Source;

fn main() {
    let output = BufWriter::new(std::io::stdout());
    let input = BufReader::new(std::io::stdin());
    let mut server = Server::new(input, output);

    let req = server.poll_request().unwrap().unwrap();
    if let Command::Initialize(_) = req.command {
        let rsp = req.success(ResponseBody::Initialize(Capabilities::default()));

        server.respond(rsp).unwrap();

        server.send_event(Event::Initialized).unwrap();
    } else {
        return;
    }

    let (request, launch) = loop {
        let request = server.poll_request();

        if let Ok(Some(request)) = request {
            if let Command::Launch(ref launch) = request.command {
                break (request.clone(), launch.clone());
            }

            let _ = request.ack();
        }
    };

    let data = launch.additional_data.unwrap();

    let file = data
        .as_object()
        .unwrap()
        .get("program")
        .unwrap()
        .as_str()
        .unwrap();

    let file = PathBuf::from(file);

    request.ack().unwrap();

    let content = fs::read_to_string(file.clone()).unwrap();

    let result = marble::execute_string(&content, file);

    let source = Source::new(&content);

    match result {
        Ok((value, output)) => {
            if !output.is_empty() {
                server
                    .send_event(Event::Output(OutputEventBody {
                        category: Some(OutputEventCategory::Stdout),
                        output,
                        ..Default::default()
                    }))
                    .unwrap();
            }

            server
                .send_event(Event::Output(OutputEventBody {
                    category: Some(OutputEventCategory::Console),
                    output: value.to_string(),
                    ..Default::default()
                }))
                .unwrap();
        }
        Err(error) => {
            server
                .send_event(Event::Output(OutputEventBody {
                    category: Some(OutputEventCategory::Stderr),
                    output: error.of_source(&source),
                    ..Default::default()
                }))
                .unwrap();
        }
    };

    server.send_event(Event::Terminated(None)).unwrap();
    server
        .send_event(Event::Exited(ExitedEventBody { exit_code: 0 }))
        .unwrap();
}
