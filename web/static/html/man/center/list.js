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
    self.pb_id = self.com.get_id("pagebar");
    //self.detail_id = self.com.get_id("modal_body");
    self.body_id = self.com.get_id("modal_body");

    self.get_page_data(1, cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var account_id = 'a[flag="' + self.com.get_id("account") + '"]';
    var edit_id = 'a[flag="' + self.com.get_id("edit") + '"]';
    var el = [
        {id:account_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_account_page(t_id);
        }},
        {id:edit_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.dom_modal.modal('show');
            var add = {
                id:t_id
            }
            new window.Com({id:self.body_id, path:"man_center_edit", pins:self, add:add});
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
    CurSite.postDigest({cmd:"ACT02"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.set_list = back_body.data;
            self.data.total = back_body.count;
            self.data.province_type = back_body.province_type;

            for(var i = 0; i < self.data.set_list.length; i++) {
                var set = self.data.set_list[i];
                set.province_des = self.data.province_type[set.province].desc;
            }
        }
        cb(null, null);
    });
}

Com.prototype.to_page = function(index, cb) {
    var self = this;
    self.data.cur_page = index;
    self.get_page_data(index, function(err, data){
        self.com.refresh()
    })
}
