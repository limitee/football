var Com = function() {
    var self = this;
    self.data = {
        game_rel: {}
    }
};

Com.prototype.init = function(cb) {
    var self = this;

    juicer.unregister('get_game_name');
    juicer.register('get_game_name', function(id){
        return self.data.game.name;
    });

    juicer.unregister('get_play_name');
    juicer.register('get_play_name', function(id){
        return self.data.play_type.name;
    });

    juicer.unregister('get_bet_name');
    juicer.register('get_bet_name', function(id){
        return self.data.bet_type.name;
    });

    juicer.unregister('get_ticket_status_des');
    juicer.register('get_ticket_status_des', function(id){
        return self.data.ticket_status[id].desc;
    });

    self.t_id = self.com.cfg.add.id;
    var body = {
        id: self.t_id
    };
    CurSite.postDigest({cmd:"ATI02"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.set = back_body.ticket;
            self.data.game_list = back_body.game_list;
            for(var key in self.data.game_list) {
                var set = self.data.game_list[key];
                if(set.id == self.data.set.game_id) {
                    self.data.game = set;
                    break;
                }
            }
            self.data.play_type = self.data.game.map[self.data.set.play_type];
            self.data.bet_type = self.data.play_type.map[self.data.set.bet_type];
            self.data.ticket_status = back_body.ticket_status;
        }
        cb(null, null);
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var subt_id = self.com.get_jid("subt");
    console.log(subt_id);
    var el = [
        {id:subt_id, on:"click", do:function(e){
            var stub = self.dom_stub.val();

            alert(stub);
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_stub = self.com.get("stub");
    cb(null, null);
}
