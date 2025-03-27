#!/usr/bin/env python3

import os
import re
from pathlib import Path

def find_prover_files(root_dir):
    """Find all prover.rs files recursively."""
    prover_files = []
    for root, _, files in os.walk(root_dir):
        for file in files:
            if file == 'prover.rs':
                prover_files.append(os.path.join(root, file))
    return prover_files

def get_module_name(file_path):
    """Get the module name from the parent directory name."""
    parent_dir = os.path.basename(os.path.dirname(file_path))
    return parent_dir

def update_mod_rs(file_path, old_name, new_name):
    """Update module declarations in mod.rs files."""
    mod_rs_path = os.path.join(os.path.dirname(file_path), 'mod.rs')
    if not os.path.exists(mod_rs_path):
        return

    with open(mod_rs_path, 'r') as f:
        content = f.read()

    # Update module declarations
    content = content.replace(f'mod {old_name};', f'mod {new_name};')
    
    # Update simple re-exports
    content = content.replace(f'pub use {old_name}::*;', f'pub use {new_name}::*;')
    
    # Update specific re-exports with curly braces
    pattern = f'pub use {old_name}::{{([^}}]+)}}'
    content = re.sub(pattern, lambda m: f'pub use {new_name}::{{{m.group(1)}}}', content)
    
    # Update any remaining prover references
    content = content.replace(f'{old_name}::', f'{new_name}::')

    with open(mod_rs_path, 'w') as f:
        f.write(content)

def main():
    # Get the crates directory path
    script_dir = os.path.dirname(os.path.abspath(__file__))
    crates_dir = os.path.join(os.path.dirname(script_dir), 'crates')

    # Find all prover.rs files
    prover_files = find_prover_files(crates_dir)

    for file_path in prover_files:
        # Get the module name from the parent directory
        module_name = get_module_name(file_path)
        
        # Create the new file path
        new_file_path = os.path.join(os.path.dirname(file_path), f'{module_name}.rs')
        
        # Update mod.rs files before renaming
        update_mod_rs(file_path, 'prover', module_name)
        
        # Rename the file
        os.rename(file_path, new_file_path)
        print(f'Renamed {file_path} to {new_file_path}')

if __name__ == '__main__':
    main() 