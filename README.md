# OpenBar-Notifier

> OpenBar-Notifier informs you about any interesting changes in the stocks of an OpenBar instance.  
> Technically, it is indeed a "scrapper" that just publish the events of interest to a WebHook URL.

## Why?

If you are an OpenBar user, you are probably also a student or at least related to studies in TELECOM Nancy.
As you may know, home-made sandwiches are made each day and sold using the OpenBar but unfortunately in 
a very limited quantity. If you are not fast enough, you may miss the opportunity to buy one of these
delicious sandwiches. 

OpenBar-Notifier will not attempt to buy a sandwich for you (because we still want something fair for everyone)
but it will notify you as soon as a new sandwich is available. You can then run to the ordering page and
place your order before someone else does it.

## How (to setup)?

This project requires a Rust compiler (at time of writing, Rust 1.90.0 is the current target). You can find
installation instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can build the project using the following command:

```bash
cargo build --release
```

This will create a release build of the project, which you can then run.

```bash
cargo run --release
```

This last command is actually a simple alias for running the compiled binary located in `./target/release/openbar-notifier`.

You just need something like a cronjob to run it periodically, for instance every 5 minutes between 8am and 11am on weekdays.

***TODO: More details about the cronjob when it is actually usable.***

## How (to develop/to contribute)?

If you want to contribute to this project, you can fork the repository and create a pull request with your changes.  
Please make sure to follow the existing code style and include tests for any new functionality you add.  
Feel free to open an issue if you find a bug or have a feature request.  

Otherwise the previous section already explains how to build and run the project.

That said, we also relies on some code-generation for the OpenBar API client.  
***TODO: More details about that.***

## License

I am aware this project is actually very simple, and it could probably be released under the MIT license without any particular issue.
However, just to ensure this project will remain free, and used for good purposes only (and prevent some automated bot to use it for auto-buying sandwiches for instance),  
I decided to release it under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).

FuseTim (2025) - All rights reserved.