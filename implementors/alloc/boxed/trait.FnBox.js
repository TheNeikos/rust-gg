(function() {var implementors = {};
implementors['shared_library'] = [];implementors['tempfile'] = [];implementors['glutin'] = [];implementors['glium'] = [];implementors['gg'] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()