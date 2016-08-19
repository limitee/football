var Com = function() {
    var self = this;
    self.data = {
        skip: 0,
        limit: 10,
        sort: [{id:-1}],
        cond: {},
        total: 0,
        cur_page: 1,
        set_list: []
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    async.waterfall([
        function(cb) {  //get center info
            self.data.job_id = CurSite.add_job(10, function() {
                self.to_page()
            })
            cb(null)
        }
    ], function(err, data) {
        self.get_page_data(1, cb);
    });
}

Com.prototype.destroy = function(cb) {
    var self = this;
    CurSite.remove_job(self.data.job_id);
    if(cb) {
        cb(null, null);
    }
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var game_id = 'button[flag="' + self.com.get_id("game") + '"]';
    var account_id = 'button[flag="' + self.com.get_id("account") + '"]';
    var charge_id = 'button[flag="' + self.com.get_id("charge") + '"]';
    var charge_report_id = 'button[flag="' + self.com.get_id("charge_report") + '"]';
    var test_id = 'button[flag="' + self.com.get_id("test") + '"]';
    var el = [
        {id:test_id, on:"click", do:function(e){
            /*
            var body = {
                id: 9, 
                bonus: 100,
                terminal: 'term_001',
                status:3 
            }
            CurSite.postDigest({cmd:"CR04"}, body, function(err, back_body)
            {
                console.log(back_body);
            });
            */
            var body = {
            }
            CurSite.postDigest({cmd:"CR03"}, body, function(err, back_body)
            {
                console.log(back_body);
            });
        }},
        {id:charge_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            var body = {
                terminal_id: t_id
            }
            CurSite.postDigest({cmd:"CRT03"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    alert("操作成功，过一段时间之后进入账号管理查看")
                }
            });
        }},
        {id:charge_report_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_charge_report_page(t_id);
        }},
        {id:account_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_account_page(t_id);
        }},
        {id:game_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_game_page(t_id);
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function (cb) {
    var self = this;

    self.dom_modal = self.com.get("myModal");
    self.dom_modal_body = self.com.get("myModalBody");
}

Com.prototype.get_page_data = function(index, cb) {
    var self = this;
    self.data.time = CurSite.getDateStr();
    var body = {
        cond: JSON.stringify(self.data.cond),
        sort: JSON.stringify(self.data.sort)
    };
    CurSite.postDigest({cmd:"CRT01"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.set_list = back_body.data;
            self.data.total = back_body.count;
        }
        cb(null, null);
    });
}

Com.prototype.to_page = function(cb) {
    var self = this;
    self.data.cur_page = 1;
    self.get_page_data(1, function(err, data){
        self.com.refresh()
    })
}
