import("../crate/pkg").then(module => {
    module.run();

    window.setInterval(module.update_time, 1000);
});

import("./app").then(module => {
    console.log('Loaded...');
});

