var Com = function() {
    var self = this;
    self.data = {
        skip: 0,
        limit: 10,
        sort: [{id:-1}],
        cond: {},
        total: 0,
        set_list: []
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    self.pb_id = self.com.get_id("pagebar");
    self.detail_id = self.com.get_id("modal_body");

    self.get_page_data(1, cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var item_id = 'a[flag="' + self.com.get_id("view_st") + '"]';
    var account_id = 'a[flag="' + self.com.get_id("account") + '"]';
    var charge_id = 'a[flag="' + self.com.get_id("charge") + '"]';
    var report_id = 'a[flag="' + self.com.get_id("report") + '"]';
    var reset_id = 'a[flag="' + self.com.get_id("reset") + '"]';
    var el = [{id:item_id, on:"click", do:function(e){
        var t_id = parseInt($(this).attr("t_id"));
        self.com.pins.to_detail_page(t_id);
        }},
        {id:reset_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_reset_page(t_id);
        }},
        {id:account_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_account_page(t_id);
        }},
        {id:charge_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_charge_page(t_id);
        }},
        {id:report_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.dom_modal.modal("show");

            var add = {
                id:t_id
            }
            new window.Com({id:self.detail_id, path:"man_center_report", pins:self, add:add});
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function (cb) {
    var self = this;
    var add = {
        skip: self.data.skip,
        limit: self.data.limit,
        total: self.data.total
    }

    self.dom_modal = self.com.get("myModal");
    self.dom_modal_body = self.com.get("myModalBody");

    new window.Com({id:self.pb_id, path:"sys_pagebar", pins:self, add:add}, function(err, data){
        cb(null, null)
    });
}

Com.prototype.get_page_data = function(index, cb) {
    var self = this;
    self.data.skip = (index - 1)*self.data.limit;
    var body = {
        cond: JSON.stringify(self.data.cond),
        sort: JSON.stringify(self.data.sort),
        offset: self.data.skip,
        limit: self.data.limit
    };
    CurSite.postDigest({cmd:"AC02"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.set_list = back_body.data;
            self.data.total = back_body.count;
        }
        cb(null, null);
    });
}

Com.prototype.to_page = function(index, cb) {
    var self = this;
    self.get_page_data(index, function(err, data){
        self.com.refresh()
    })
}
