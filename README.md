# MP3-Metadata-Extractor
A small program to print Metadata of MP3 file using Rust programming language.

For system requirements please refer file 'Application_Requirements.txt'.

How to run program.

1. Clone the repository.
2. navigate inside folder 'mp3_metadata_extractor'
3. cargo build
4. cargo run Sample.mp3

To save output to a file execute following command.
 
 cargo run Sample.mp3 > Sample.txt
 
 ------------------
 
 Output :- 
 
 Output Screenshot 1
 
 ![Output1](https://user-images.githubusercontent.com/86361080/233912956-3e2481bd-c528-4839-b09c-ec9b47b44534.png)

 
 Output Screenshot 2
 
 ![Output2](https://user-images.githubusercontent.com/86361080/233913580-ee51414e-3035-4c7a-96d7-8e21ceeb927d.png)


--------------------

__Project Tree Structure__

```

mp3_metadata_extractor v0.1.0 () - 
`-- mp3-metadata feature "default"
    `-- mp3-metadata v0.3.4 (https://github.com/GuillaumeGomez/mp3-metadata) - 

```

__Repository Tree Structure__

```
├── .github
    └── workflows
    │   └── rust.yml
├── Application_Requirements.txt
├── LICENSE
├── OUTPUT
    ├── Output1.png
    └── Output2.png
├── README.md
└── mp3_metadata_extractor
    ├── .gitignore
    ├── Cargo.lock
    ├── Cargo.toml
    ├── Sample.mp3
    ├── Sample.txt
    └── src
        └── main.rs

```
