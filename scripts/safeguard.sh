#!/bin/bash
# Safeguard script to validate tool inputs before execution

function validate_replace() {
    local file="$1"
    local instruction="$2"
    local old="$3"
    local new="$4"
    
    if [[ -z "$file" || -z "$instruction" || -z "$old" || -z "$new" ]]; then
        echo "ERROR: Missing required parameters for replace tool."
        return 1
    fi
    return 0
}

function validate_shell() {
    local command="$1"
    if [[ -z "$command" ]]; then
        echo "ERROR: Missing command for run_shell_command."
        return 1
    fi
    return 0
}

# Add more validations as needed
