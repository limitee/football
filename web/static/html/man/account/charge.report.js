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

    juicer.unregister('get_status_des');
    juicer.register('get_status_des', function(id){
        return self.data.charge_report_status[id].desc;
    });

    self.pb_id = self.com.get_id("pagebar");
    self.com_id = self.com.cfg.add.id;
    async.waterfall([
        function(cb) {
            var body = {
                id: self.com_id
            };
            CurSite.postDigest({cmd:"AA03"}, body, function(err, back_body)
            {

                if(back_body) {
                    self.data.set = back_body.customer;
                    self.data.account = back_body.account;
                }
                cb(null);
            });
        }
    ], function(err, data){
        self.get_page_data(1, cb);
    })
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var charge_id = 'a[flag="' + self.com.get_id("charge") + '"]';
    var ignore_id = 'a[flag="' + self.com.get_id("ignore") + '"]';
    var el = [
        {id:charge_id, on:"click", do:function(e){
            var report_id = parseInt($(this).attr("t_id"));
            var terminal_id = self.com_id;
            var body = {
                report_id: report_id,
                terminal_id: terminal_id,
                flag: 1
            }
            CurSite.postDigest({cmd:"AA07"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    self.refresh();
                }
            });
        }},
        {id:ignore_id, on:"click", do:function(e){
            var report_id = parseInt($(this).attr("t_id"));
            var terminal_id = self.com_id;
            var body = {
                report_id: report_id,
                terminal_id: terminal_id,
                flag: 0
            }
            CurSite.postDigest({cmd:"AA07"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    self.refresh();
                }
            });
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
        limit: self.data.limit,
        id: self.com_id
    };
    CurSite.postDigest({cmd:"AA06"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.set_list = back_body.list.data;
            self.data.total = back_body.count;

            self.data.charge_report_status = back_body.charge_report_status;
        }
        cb(null, null);
    });
}

Com.prototype.refresh = function(cb) {
    var self = this;
    self.init(function(err, data){
        self.com.refresh()
    });
}

Com.prototype.to_page = function(index, cb) {
    var self = this;
    self.get_page_data(index, function(err, data){
        self.com.refresh()
    })
}