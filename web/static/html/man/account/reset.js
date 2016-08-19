var Com = function() {
    var self = this;
};

Com.prototype.init = function(cb) {
    var self = this;
    self.com_id = self.com.cfg.add.id;
    var body = {
        id: self.com_id
    };
    CurSite.postDigest({cmd:"AA03"}, body, function(err, back_body)
    {

        if(back_body) {
            self.data.set = back_body.customer;
        }
        cb(null, null);
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;

    var sbt_id = self.com.get_jid("sub");
    var el = [{id:sbt_id, on:"click", do:function(e){
        var bt = $(this);
        bt.button("loading");
        var data = self.get_data();
        if(self.check(data)) {
            self.save(data, function(err, data){
                bt.button("reset");

                self.com.pins.to_account_page(self.com_id);
            });
        } else {
            bt.button("reset");
        }
    }}];
    cb(null, el);
}


Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_amount = self.com.get("amount");
    self.dom_order_id = self.com.get("order_id");
    cb(null, null)
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.amount = self.dom_amount.val();
    data.order_id = self.dom_order_id.val();
    return data;
}

Com.prototype.check = function(data) {
    var self = this;
    if(data.amount.length == 0) {
        alert("金额格式错误");
        return false;
    }
    if(data.order_id.length == 0) {
        alert("订单id格式错误");
        return false;
    }
    return true;
}

Com.prototype.save = function(data, cb) {
    var self = this;
    data.id = self.com_id;
    data.amount = parseInt(parseFloat(data.amount)*100);
    var body = data;
    CurSite.postDigest({cmd:"AA08"}, body, function(err, back_body)
    {
        if(err) {
            alert(err);
        } else {
            alert("操作成功")
        }
        if(cb) {
            cb(null, null);
        }
    });
}