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
        return self.data.game_rel[id];
    });
    
    self.com_id = self.com.cfg.add.id;
    self.get_page_data(cb)
}

Com.prototype.get_page_data = function(cb) {
    var self = this;
    var body = {
        id: self.com_id
    };
    CurSite.postDigest({cmd:"AT04"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.terminal_game_list = back_body.terminal_game_list.data;
            self.data.game_list = back_body.game_list;
            
            if(self.data.game_list.length > 0) {
                self.data.game_id = self.data.game_list[0].id;
            }
            
            for(var key in self.data.game_list) {
                var set = self.data.game_list[key];
                self.data.game_rel[set.id] = set.name;
            }
        }
        cb(null, null);
    });
}

Com.prototype.get_scale_by_id = function(id) {
    var self = this;
    for(var key in self.data.terminal_game_list) {
        var set = self.data.terminal_game_list[key];
        if(set.id == id) {
            return set.scale;
        }
    }
    return -1;
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    
    var sbt_id = self.com.get_jid("sub");
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';
    var delete_id = 'a[flag="' + self.com.get_id("delete") + '"]';
    var scale_id = 'a[flag="' + self.com.get_id("set_scale") + '"]';
    var save_id = self.com.get_jid("save");
    var el = [
        {id:save_id, on:"click", do:function(e){
            var id = parseInt(self.dom_cur_id.html());
            var scale = self.dom_scale.val();
            scale = parseInt(parseFloat(scale)*1000);
            var data = {
                id: id,
                scale: scale
            };
            var body = {
                data: data 
            };
            CurSite.postDigest({cmd:"AT13"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    self.dom_modal.modal('hide');
                }
            });
        }},
        {id:sbt_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var data = self.get_data();
            if(self.check(data)) {
                self.save(data, function(err, data){
                    self.refresh()
                    //bt.button("reset");
                });
            } else {
                bt.button("reset");
            }
        }},
        {id:bar_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.game_id = parseInt($(this).attr("t_id"));
        }},
        {id:delete_id, on:"click", do:function(e){
	        var t_id = parseInt($(this).attr("t_id"));
	        self.delete_terminal_game(t_id);
	    }},
        {id:scale_id, on:"click", do:function(e){
	        var t_id = parseInt($(this).attr("t_id"));
            self.dom_cur_id.html(t_id);
            var scale = self.get_scale_by_id(t_id);
            self.dom_scale.val(scale/1000);
            self.dom_modal.modal("show");
	    }}
    ];
    cb(null, el);
}


Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_modal = self.com.get("modal");
    self.dom_modal.on('hidden.bs.modal', function (e) {
        self.refresh();
    });
    self.dom_cur_id = self.com.get("cur_id");
    self.dom_scale = self.com.get("scale");
    cb(null, null);
}

Com.prototype.refresh = function() {
    var self = this;
    self.get_page_data(function(err, data) {
        self.com.refresh()
    });
}

Com.prototype.delete_terminal_game = function(id) {
    var self = this;
    if(confirm("确定要删除此条记录？")) {
        var body = {
		    id: id
		};
		CurSite.postDigest({cmd:"AT06"}, body, function(err, back_body)
		{
		    self.refresh();
		});
    }
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.game_id = self.data.game_id;
    data.terminal_id = self.com_id;
    return data;
}

Com.prototype.check = function(data) {
    var self = this;
    return true;
}

Com.prototype.save = function(data, cb) {
    var self = this;
    var body = data;
    CurSite.postDigest({cmd:"AT05"}, body, function(err, back_body)
    {
        //alert("操作成功")
        if(cb) {
            cb(null, null);
        }
    });
}
