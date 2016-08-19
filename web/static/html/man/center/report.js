var Com = function() {
    var self = this;
    self.data = {
        game_rel: {},
        set_list: []
    }
};

Com.prototype.init = function(cb) {
    var self = this;

    juicer.unregister('get_game_name');
    juicer.register('get_game_name', function(id){
        return self.data.game_rel[id].name;
    });

    juicer.unregister('get_play_name');
    juicer.register('get_play_name', function(set){
        return self.data.game_rel[set.game_id].map[set.play_type].name;
    });

    juicer.unregister('get_bet_name');
    juicer.register('get_bet_name', function(set){
        return self.data.game_rel[set.game_id].map[set.play_type].map[set.bet_type].name;
    });

    self.t_id = self.com.cfg.add.id;
    var body = {
        id: self.t_id
    };
    CurSite.postDigest({cmd:"ACT03"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.set_list = back_body.data.data;

            self.data.game_list = back_body.game_list;
            for(var key in self.data.game_list) {
                var set = self.data.game_list[key];
                self.data.game_rel[set.id] = set;
            }
        }
        cb(null, null);
    });
}

