# NRF24-MultiWii-Drone

This repository contains both software and electrical components for building a drone using the NRF24L01 radio modules and Arduino.

![Prototype One](docs/assets/prototype_1_drone.jpg)

# Changelog

* Jun 08, 2025 [v1.0]: Prototype 1.0 [Fail] - More details for this initial prototype in the documentation.
* Jul 22, 2025 [v1.1]: Prototype 1.1 [Fail] - More details for this second prototype in the documentation.

# Schematics and Diagrams

The following schematic diagram shows the wiring components and hardware for the drone.

![Drone Schematic](docs/assets/schematic_drone.png)

The following schematic diagram shows the wiring components and hardware for the drone controller. 

![Controller Schematic](docs/assets/schematic_controller.png)

The following blueprint shows the materials needed to build the frame for the drone. The PDF version of this blueprint can be found and printed under `docs/assets/carbon-fiber-blueprint-drone-frame.pdf`.

![Drone Frame Blurprint](docs/assets/carbon-fiber-blueprint-drone-frame.png)

The tutorial for building and assembling the drone can be found under `\docs`.

# Documentation

The documentation that provides more details and tutorials on building the drone can be found under `\docs`. 

To generate the documentation using `sphinx`, follow the tutorial below with the command line. 

```shell
cd docs
```

```shell
pip install -r requirements.txt
```

```shell
make clean
make html
```

```shell
sudo apt-get install latexmk
sudo apt-get install texlive-fonts-recommended
sudo apt-get install texlive-latex-extra
```

```shell
make latexpdf
```

# Credits

Original drone software is credited to [iforce2d](https://www.youtube.com/@iforce2d).

## Drone Tutorial - Max Imagination

[![Watch the video](https://img.youtube.com/vi/Sa6EslOHsI0/0.jpg)](https://www.youtube.com/watch?v=Sa6EslOHsI0)

* [Elektor Labs blog post](https://www.elektormagazine.com/labs/make-a-tiny-arduino-drone-with-fpv-camera)
* [Project Files](https://drive.google.com/drive/folders/1mWTCPN2daOcmTa4wUF0j8qPXhXvxLfkK)

## Drone Tutorial - ElectroNoobs

[![Watch the video](https://img.youtube.com/vi/J0x4ChjUS00/0.jpg)](https://www.youtube.com/watch?v=J0x4ChjUS00&t=719s)

