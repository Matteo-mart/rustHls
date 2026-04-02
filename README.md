## Fonctionnement Technique

Le programme fonctionne en trois étapes :

    Réception : prend une vidéo via la commande

    Traitement : utilise FFmpeg pour traiter la vidéo

    Indexation : génère un fichier .m3u8 qui sert de sommaire pour la lecture

### Pré-requis

    Rust & Cargo (pour compiler le projet).

    FFmpeg installé sur la machine.

## Installation et Test

    Clonage du dépôt :
    Bash

        git clone https://github.com/Matteo-mart/rustHls.git
        cd rustHls

    Compilation :
    Bash

        cargo build

    Exécution :
    Pour tester avec un fichier video.mp4 :
    Bash

        clear && cargo run video/hd.mp4
        clear && cargo run video/groovy-all-videos-and-all-audios.mp4

## Structure du code

    main.rs : Point d'entrée de l'application et gestion des arguments.

    segmenter.rs : Logique de découpage et gestion des processus enfants.

    playlist.rs : Génération du texte au format M3U8.

