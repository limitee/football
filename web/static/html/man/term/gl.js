var Com = function() {
    var self = this;
    self.data = {
        bonus_map: {},
        tax_map: {}
    }
};

Com.prototype.init = function(cb) {
    var self = this;
    self.data.term_id = self.com.cfg.add.term_id;
    self.data.term_code = self.com.cfg.add.term_code;
    self.data.game_id = self.com.cfg.add.game_id;

    self.get_page_data(cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;

    var sbt_id = self.com.get_jid("sub");
    var next_id = self.com.get_jid("next"); //跳转到录入开奖号码页面
    var get_info_id = self.com.get_jid("get_info"); //抓取号码 
    var el = [
        {id:sbt_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var back_list = self.get_upload_list();
            self.save(back_list);
            bt.button("reset");
        }},
        {id:next_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            self.com.pins.to_draw_page(self.data.term_id, self.data.term_code, self.data.game_id);
        }},
        {id:get_info_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var url = self.dom_url.val();
            if(url) {
                var body = {
                    term_id: self.data.term_id,
                    url: url
                };
                CurSite.postDigest({cmd:"AGT08"}, body, function(err, back_body)
                {
                    if(err) {
                        alert(err.des);
                    } else {
                        alert("操作成功");
                    }
                    //bt.button("reset");
                    self.refresh();
                });
            } else {
                alert("请填写url");
                bt.button("reset");
            }
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_url = self.com.get("url");
    cb(null, null)
}

Com.prototype.get_page_data = function(cb) {
    var self = this;
    var body = {
        term_id: self.data.term_id,
        term_code: self.data.term_code,
        game_id: self.data.game_id
    };
    CurSite.postDigest({cmd:"AGT05"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.gl_list = back_body.gl_list;
            self.set_true_list(back_body.gl_list);
            self.data.db_list = back_body.db_list;
            self.set_true_list(back_body.db_list);
        }
        cb(null, null);
    });
}

Com.prototype.set_true_list = function(list) {
    var self = this;
    for(var i = 0; i < list.length; i++) {
        var set = list[i];
        self.data.bonus_map[set.lev] = set.bonus;
        self.data.tax_map[set.lev] = set.bonus_after_tax;
    }
}

/**
 * 获得需要发送到服务端的数据
 */
Com.prototype.get_upload_list = function() {
    var self = this;
    var back_list = [];
    for(var i = 0; i < self.data.gl_list.length; i++) {
        var set = self.data.gl_list[i];

        var dom_bonus = self.com.get("lev_bonus_" + set.lev);
        var bonus = parseInt(dom_bonus.val());

        var dom_tax = self.com.get("lev_tax_" + set.lev);
        var tax = parseInt(dom_tax.val());

        var obj = {
            lev: set.lev,
            bonus: bonus*100,
            bonus_after_tax: tax*100,
            game_id: self.data.game_id,
            term_code: self.data.term_code,
            descrip: set.descrip
        }
        back_list.push(obj);
    }
    return back_list;
}

Com.prototype.save = function(list) {
    var self = this;
    var body = {
        gl_list: list
    };
    CurSite.postDigest({cmd:"AGT06"}, body, function(err, back_body)
    {
        if(err) {
            alert(err);
        } else {
            alert("保存成功，请确认下面列表的正确性，并进行下一步");
        }
        self.refresh();
    });
}

Com.prototype.refresh = function() {
    var self = this;
    self.get_page_data(function(err, data) {
        self.com.refresh()
    });
}
