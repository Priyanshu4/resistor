# resistor

resistor is a simple command line utility to get the resistance, tolerance and temperature sensitivity of color coded resistors 

resistor takes the color bands of the resistor as arguments

Examples:
```
$ resistor brown red brown red
120Ω ±2%
```
```
$ resistor brown black orange red gold
10.3kΩ ±5%
```
```
$ resistor brown white blue brown brown red
1.96kΩ ±1% 50 ppm/C
```
For help, run resistor with `-h`

resistor can be installed by cloning this repo and running 
`cargo install --path .` in the directory of this repo
