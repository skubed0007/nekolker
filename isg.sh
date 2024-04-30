#!/bin/bash

# Define the installation directory
LINUX_INSTALL_DIR=/usr/local/bin

# Define the name of the executable
EXECUTABLE_NAME=./nekolker

# Function to generate the Linux installation script with sudo privileges
generate_linux_install_script() {
    cat <<EOT
#!/bin/bash
sudo su
echo "Installing $EXECUTABLE_NAME to $LINUX_INSTALL_DIR"
sudo cp "$EXECUTABLE_NAME" "$LINUX_INSTALL_DIR"
echo "Installation complete"
exit
EOT
}

# Generate the Linux installation script with sudo privileges
generate_linux_install_script > install_linux.sh
chmod +x ./install_linux.sh
echo "Linux installation script generated as install_linux.sh"
