day day: 
    #!/bin/bash
    day="$( python -c 'day=int("{{day}}");print(f"{day:02}")' )"
    set -e
    echo "Creating day ${day}"
    set -x
    mkdir "src/day${day}"
    liquidjs -t @src/template/mod.rs \
        -c "{ \"day\": \"{{ day }}\" }" \
        -o "src/day${day}/mod.rs"
    sed -i.bak "/.*day${day} =.*/s/^\(\s*\)\/\/\s/\1/g" src/lib.rs
    sed -i.bak "/.*day${day},/s/^\(\s*\)\/\/\s/\1/g" benches/criterion.rs
