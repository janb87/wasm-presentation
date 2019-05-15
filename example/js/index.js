import("../crate/pkg").then(module => {
    module.run();
});

import("./app").then(module => {
    console.log('Loaded...');
});

