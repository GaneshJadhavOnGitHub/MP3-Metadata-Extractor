# MP3-Metadata-Extractor
A small program to print Metadata of MP3 file using Rust programming language.

For system requirements please refer file 'Application_Requirements.txt'.

How to run program.

1. Clone the repository.
2. navigate inside folder 'mp3_metadata_extractor'
3. cargo build
4. cargo run Ae_Ajnabi_Tu_Bhi_Kahin_Awaz_De.mp3

To save output to a file execute following command.
 
  % cargo run -- Ae_Ajnabi_Tu_Bhi_Kahin_Awaz_De.mp3 > Output.txt

 
 ------------------
 
 Output :- 
 
 Output Screenshot 1
 
 
 ![Output1](https://github.com/GaneshJadhavOnGitHub/MP3-Metadata-Extractor/blob/main/OUTPUT/Output1.png?raw=true)

 
 Output Screenshot 2
 
 ![Output2](https://github.com/GaneshJadhavOnGitHub/MP3-Metadata-Extractor/blob/main/OUTPUT/Output2.png?raw=true)


Output Screenshot 3
 
 ![Output3](https://github.com/GaneshJadhavOnGitHub/MP3-Metadata-Extractor/blob/main/OUTPUT/Output3.png?raw=true)

--------------------

__Project Dependency Tree__

```

mp3_metadata_extractor v0.1.0 () - 
`-- mp3-metadata feature "default"
    `-- mp3-metadata v0.3.4 (https://github.com/GuillaumeGomez/mp3-metadata) - 

```

__Repository Tree Structure__

```
 % tree -I '.git' -a 
.
├── Application_Requirements.txt
├── .github
│   └── workflows
│       └── rust.yml
├── LICENSE
├── mp3_metadata_extractor
│   ├── Ae_Ajnabi_Tu_Bhi_Kahin_Awaz_De.mp3
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── .gitignore
│   ├── Output.txt
│   └── src
│       └── main.rs
├── OUTPUT
│   ├── Output1.png
│   ├── Output2.png
│   └── Output3.png
└── README.md

5 directories, 13 files


```
