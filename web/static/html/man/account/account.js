var Com = function() {
    var self = this;
    self.data = {
        skip: 0,
        limit: 10,
        sort: [{id:-1}],
        cond: {},
        add: {},
        total: 0,
        set_list: [],
        moneylog_type: [],
        time_list:[
            {id:-1, des:"当前"}
        ],
        cur_time: -1
    }
}

Com.prototype.init = function(cb) {
    var self = this;

    var date = CurSite.getDateStr(); 
    self.data.add.start_time = date.substr(0, 10) + " 00:00:00";
    self.data.add.end_time = date.substr(0, 10) + " 00:00:00";

    juicer.unregister('get_moneylog_type_des');
    juicer.register('get_moneylog_type_des', function(id){
        return self.data.moneylog_type[id].desc;
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
    var item_id = 'a[flag="' + self.com.get_id("view_st") + '"]';
    var search_id = self.com.get_jid("search");
    var time_item_id = 'li[flag="' + self.com.get_id("time_item") + '"]';
    var el = [
        {id:time_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.cur_time = parseInt($(this).attr("t_id"));
        }},
        {id:item_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_detail_page(t_id);
        }},
        {id:search_id, on:"click", do:function(e){
            self.data.cur_page = 1;
            var type_id = parseInt(self.dom_type_id.val());
            if(type_id > 0) {
                self.data.cond.type = type_id;
            } else {
                delete self.data.cond.type;
            }
            var order_id = self.dom_order_id.val();
            if(order_id) {
                self.data.cond.order_id = order_id;
            } else {
                delete self.data.cond.order_id;
            }
            self.refresh();
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

    self.dom_type_id = self.com.get("type_id");
    self.dom_order_id = self.com.get("order_id");

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
        cond: self.data.cond,
        op: {
            sort: self.data.sort,
            offset: self.data.skip,
            limit: self.data.limit
        },
        id: self.com_id,
        cur_time: self.data.cur_time
    };
    CurSite.postDigest({cmd:"AA04"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.set_list = back_body.moneylog.data;
            self.data.total = back_body.count;
            
            var time_list = [{id:-1, des:"当前"}];
            var year = back_body.cur_year;
            var month = back_body.cur_month;
            for(var i = 0; i < 5; i++) {
                var time_info = self.get_cur_time(year, month); 
                if(time_info) {
                    time_list.push(time_info);
                }
                month -= 1;
                if(month == 0) {
                    month = 12;
                    year -= 1;
                }
            }
            self.data.time_list = time_list;

            self.data.moneylog_type = back_body.moneylog_type;
        }
        cb(null, null);
    });
}

Com.prototype.get_cur_time = function(year, month) {
    if(year <= 2016 && month < 5) {
        return null;
    }
    var time_info = {id: year*100 + month, des:(year*100 + month) + ""};
    return time_info;
}

Com.prototype.refresh = function() {
    var self = this;
    self.to_page(self.data.cur_page, function(err, data){

    });
}

Com.prototype.to_page = function(index, cb) {
    var self = this;
    self.data.cur_page = index;
    self.get_page_data(index, function(err, data){
        self.com.refresh()
    })
}
