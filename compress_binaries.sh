#!/bin/bash
for file in bin/*; do
    if [ -f "$file" ]; then
        echo "Processing $file..."
        # Compress
        upx "$file"
        # Test the compressed version
        if upx -t "$file"; then
            echo "✓ $file compressed successfully"
        else
            echo "✗ $file failed test, reverting..."
            upx -d "$file"
        fi
    fi
done
