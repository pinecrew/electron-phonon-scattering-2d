var searchIndex = {};
searchIndex['electron_phonon_scattering_2d'] = {"items":[[5,"create_ensemble","electron_phonon_scattering_2d","",null,{"inputs":[{"name":"usize"},{"name":"t"},{"name":"f64"},{"name":"u32"}],"output":{"name":"vec"}}],[0,"material","","",null,null],[3,"BrillouinZone","electron_phonon_scattering_2d::material","Representation of tetragonal brillouin zone of 2D material.\na, b, c, and d are vertices of tetragon.",null,null],[12,"a","","",0,null],[12,"b","","",0,null],[12,"c","","",0,null],[12,"d","","",0,null],[12,"basis","","",0,null],[12,"dual_basis","","",0,null],[8,"Material","","",null,null],[10,"energy","","Energy spectrum of electrons",1,{"inputs":[{"name":"material"},{"name":"point"}],"output":{"name":"f64"}}],[11,"energy_polar","","Energy spectrum in polar coordinates",1,{"inputs":[{"name":"material"},{"name":"f64"},{"name":"f64"}],"output":{"name":"f64"}}],[10,"energy_gradient","","Gradient of energy in momentum space",1,{"inputs":[{"name":"material"},{"name":"point"}],"output":{"name":"vec2"}}],[10,"velocity","","",1,{"inputs":[{"name":"material"},{"name":"point"}],"output":{"name":"vec2"}}],[10,"min_energy","","Minimum of energy in brillouin zone",1,{"inputs":[{"name":"material"}],"output":{"name":"f64"}}],[10,"max_energy","","Maximum of energy in brillouin zone",1,{"inputs":[{"name":"material"}],"output":{"name":"f64"}}],[10,"momentums","","Solves equation energy_polar(p, theta) = energy",1,{"inputs":[{"name":"material"},{"name":"f64"},{"name":"f64"}],"output":{"name":"vec"}}],[10,"brillouin_zone","","brillouin zone structure",1,{"inputs":[{"name":"material"}],"output":{"name":"brillouinzone"}}],[10,"optical_energy","","optical phonon energy",1,{"inputs":[{"name":"material"}],"output":{"name":"f64"}}],[10,"optical_scattering","","optical phonon scattering probability",1,{"inputs":[{"name":"material"},{"name":"point"}],"output":{"name":"f64"}}],[10,"acoustic_scattering","","acoustic phonon scattering probability",1,{"inputs":[{"name":"material"},{"name":"point"}],"output":{"name":"f64"}}],[11,"clone","","",0,{"inputs":[{"name":"brillouinzone"}],"output":{"name":"brillouinzone"}}],[11,"new","","",0,{"inputs":[{"name":"brillouinzone"},{"name":"point"},{"name":"point"},{"name":"point"}],"output":{"name":"brillouinzone"}}],[11,"to_first_bz","","Returns equivalent momentum in first brillouin zone",0,{"inputs":[{"name":"brillouinzone"},{"name":"point"}],"output":{"name":"point"}}],[11,"pmax","","Calculates maximum value of momentum in direction $\\theta$ in first brillouin zone",0,{"inputs":[{"name":"brillouinzone"},{"name":"f64"}],"output":{"name":"f64"}}],[0,"boltzmann","electron_phonon_scattering_2d","Provides function for creating ensembles of particles with Boltzmann distribution",null,null],[3,"BoltzmannDistrib","electron_phonon_scattering_2d::boltzmann","",null,null],[11,"new","","Create new BoltzmannDistrib object for given material and temperature",2,{"inputs":[{"name":"boltzmanndistrib"},{"name":"f64"},{"name":"t"}],"output":{"name":"boltzmanndistrib"}}],[11,"make_dist","","Make ensemble of n particles with Boltzmann distribution",2,{"inputs":[{"name":"boltzmanndistrib"},{"name":"u32"},{"name":"usize"}],"output":{"name":"vec"}}],[0,"particle","electron_phonon_scattering_2d","Particle one particle movement in material under electromagnetic fields with phonon scattering",null,null],[3,"Summary","electron_phonon_scattering_2d::particle","",null,null],[12,"average_speed","","",3,null],[12,"acoustic","","",3,null],[12,"optical","","",3,null],[12,"tau","","",3,null],[3,"Particle","","",null,null],[11,"clone","","",3,{"inputs":[{"name":"summary"}],"output":{"name":"summary"}}],[11,"new","","",3,{"inputs":[{"name":"summary"},{"name":"vec2"},{"name":"u32"},{"name":"u32"},{"name":"f64"}],"output":{"name":"summary"}}],[11,"empty","","",3,{"inputs":[{"name":"summary"}],"output":{"name":"summary"}}],[11,"new","","",4,{"inputs":[{"name":"particle"},{"name":"t"},{"name":"point"},{"name":"u32"}],"output":{"name":"particle"}}],[11,"run","","",4,{"inputs":[{"name":"particle"},{"name":"f64"},{"name":"f64"},{"name":"fields"}],"output":{"name":"summary"}}],[0,"fields","electron_phonon_scattering_2d","",null,null],[3,"Fields","electron_phonon_scattering_2d::fields","Electromagnetic fields",null,null],[12,"e","","Amplitudes of constant, first and second wave electric fields",5,null],[12,"b","","Amplitudes of z-component constant, first and second wave magnetic fields",5,null],[12,"omega","","Frequences of waves. First value is unused and exists for consistency",5,null],[12,"phi","","",5,null],[11,"clone","","",5,{"inputs":[{"name":"fields"}],"output":{"name":"fields"}}],[11,"new","","",5,null],[11,"zero","","",5,{"inputs":[{"name":"fields"}],"output":{"name":"fields"}}],[0,"stats","electron_phonon_scattering_2d","",null,null],[3,"Stats","electron_phonon_scattering_2d::stats","",null,null],[12,"current","","",6,null],[12,"current_std","","",6,null],[12,"optical","","",6,null],[12,"acoustic","","",6,null],[12,"tau","","",6,null],[8,"Mean","","",null,null],[16,"Output","","",7,null],[10,"mean","","",7,{"inputs":[{"name":"mean"}],"output":{"name":"output"}}],[8,"MeanStd","","",null,null],[16,"Output","","",8,null],[10,"mean_std","","",8,{"inputs":[{"name":"meanstd"}],"output":{"name":"output"}}],[11,"from_ensemble","","",6,null],[0,"probability","electron_phonon_scattering_2d","Provides function for calculate probability of electron-phonon scattering",null,null],[5,"probability","electron_phonon_scattering_2d::probability","Calculates $\\int\\limits\\_{BZ} \\delta(E(p)-E) d\\^{2} p$",null,{"inputs":[{"name":"f64"},{"name":"t"},{"name":"f64"}],"output":{"name":"f64"}}]],"paths":[[3,"BrillouinZone"],[8,"Material"],[3,"BoltzmannDistrib"],[3,"Summary"],[3,"Particle"],[3,"Fields"],[3,"Stats"],[8,"Mean"],[8,"MeanStd"]]};
initSearch(searchIndex);
