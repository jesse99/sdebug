sdebug is a GUI debugger for the [score](https://crates.io/crates/score) discrete event simulator.
The debugger is a javascript app which uses a REST server embedded within score. sdebug has four tabs:
* The *map* tab provides a graphical representation of the root components within a simulation.
* The *log* tab shows the logs emitted by components.
* The *state* tab shows the current values for the store. Values can be changed by clicking on them.
* The *components* tab shows a hierarchical list of all components.

Clicking on a component in the map or components tabs will pop up a new browser window showing information
for just that component.

To use sdebug you need to startup a simulation with a path to sdebug's main page:
	./target/debug/examples/battle_bots --home=../sdebug/html/home.html

And then navigate to the URL below using a web browser:
	http://127.0.0.1:9000/
