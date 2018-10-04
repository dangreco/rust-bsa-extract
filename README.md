# rust-bsa-extract

This utility aims to increase the speed at which Bethesda Softworks Archives are extracted. 
Current implementations of this are either slow or not as straightforward.

This tool is currently only compatible with <strong>BSA v103</strong>, which is used in The Elder Scrolls IV: Oblivion.

The inspiration for this project was wanting to extract all of the voices from the game to sift through and find the worst
voice acting moments. I highly recommend <strong>nqdwilderness_infogeneral_0018bcc9_1.mp3</strong>. 

## Installation

You can install this utility by running the following command:

````bash
cargo install rust-bsa-extract
````

Ensure that your Rust executables are in your PATH variable.

Alternatively, you can download the latest release as a pre-compiled executable.

## Usage

To use, execute the following command:

````bash
  rust-bsa-extract /path/to/file.bsa /path/to/output/directory
````

## Contributing

Feel free to contribute to this project. For larger changes, open an issue first describing what you
will be planning on implementing. Smaller changes can be supplied in a pull request. In your PR,
describe what changes were made and why they are necessary. I'll review the PR within a few days, and leave
feedback/merge the request.