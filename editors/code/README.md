# Goboscript Extension for Visual Studio Code

This directory contains the source code for the Goboscript extension for Visual Studio Code. The extension provides syntax highlighting, code completion, and other features for Goboscript development.

> **Note:** This is the GoboScript JS extension, which is a fork of the main Goboscript extension. It is designed to work with the JavaScript version of the Goboscript compiler. For the C like version of the extension, see the main repository.

## Contributing

To develop the extension, open the `editors/code` directory in Visual Studio Code.

```sh
cd editors/code
npm install
```

Go to the Run and Debug view and select `Launch`. This will open a new instance of VS
Code with the extension enabled.

## Installation from Source

To install the extension from source, open the `editors/code` directory in Visual Studio
Code.

```sh
cd editors/code
npm install
npm run package
```

Then in Visual Studio Code, press `Ctrl` + `Shift` + `P` and run the
`Extensions: Install from VSIX...` command.
Select the `editors/code/goboscript-js.vsix` file to install the extension.
