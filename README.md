# repdp
Reasonably Efficient Portal Demo Parser written in Rust (sequel to iipdp)

## about
repdp is a rewrite of iipdp (my previous demo parser) that aims to have cleaner and faster code

i'm aiming to not make the same mistakes i made in iipdp and to make this parser more complete than iipdp was

## building
requires you have rust and cargo installed

literally just `cargo build` after you clone the repo

## running
for now it's this:

`repdp.exe <demo name> > out.txt` (creates a VERY BIG out.txt)

this is because currently i just do `println!("{demo:#?}")` for dumping (lmao)

this is why you might think that this parser is really slow. it is not (that) slow. printing 630 thousand lines is slow.

## major TODOs
- stringtables packet
- datatables packet
- proper dumping
- actual output (header info, time, etc.)
- more macros / improved AutoParse macro to eliminate most code repetition

## dependencies
`macros` [internal](./macros/) crate for proc macros

`anyhow` - for easy error handling

`once_cell` - easy to make global static variables

## alternatives and resources used
this is definitely not the most complete portal demo parser out there, see these alternatives:

- UntitledParser - https://github.com/UncraftedName/UntitledParser
- sdp-c - https://github.com/evanlin96069/sdp-c
- demogobbler - https://github.com/lipsanen/demogobbler

reading these parsers' code made me understand demos a lot better and got me information that the [dem.nekz.me](https://dem.nekz.me) didn't have / had gotten wrong