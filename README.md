# charbon 
<em>It's a proof of concept</em>

`charbon` is a modulable dashboard on your terminal made in [Rust](https://www.rust-lang.org). The main goal is to create an open-source dashboard with many modules to fit with all different developer needs.
Running on `Linux` and `macOS`.

## Configuration
When you run `charbon` for the first time, you will have a template file call `charbon.yaml` in your `~/$HOME/.config`. This file runs a default dashboard configuration, you will need to modify it to fit with your needs.
```
---
tick_rate:  250
layout:
	# Define new block
	- block:
	  # Run the "cpu" module, find in the "features" folder. 
	  app_name: "cpu"
	  # This block will take 80 percentages of the height of your terminal.
	  size: 80
	# Define another block
	- block:
	  app_name: "system_stat"
	  size: 20
```
