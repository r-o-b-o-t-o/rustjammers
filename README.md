Version Unity: 2017.3.0f3

Rust stable minimum 1.27
(penser à build en release pour les performances (`cargo build --release`))

Scene à lancer au démarrage: `Menu.unity`

Créer un fichier de Q-values pour le tabular Q-learning agent :
```sh
$ cargo run --release --bin generate_qvalues 5000
```
(Où le 5000 est le nombre de simulations. Si la valeur n'est pas spécifiée, une valeur par défaut sera utilisée.)

| Pseudo GitHub  | Élève          |
| -------------- | -------------- |
| Kryod          | Yohann JERRAIN |
| Elyrioss       | Esteban DUMAS  |
| robot0         | Axel COCAT     |
