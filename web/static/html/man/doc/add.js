var Com = function() {
    var self = this;
};

Com.prototype.init = function(cb) {
    var self = this;

    self.doc_id = -1;
    if(self.com.cfg.add && self.com.cfg.add.doc_id) {
        self.doc_id = self.com.cfg.add.doc_id;
    }

    if(self.doc_id > 0) {
        var body = {
            id: self.doc_id
        }
        CurSite.postUnDigest({cmd:"AD03"}, body, function(err, back_body)
        {
            if(back_body && back_body.data.length > 0) {
                self.data.set = back_body.data[0];
                cb(null, null)
            } else {
                cb(null, null)
            }
        });
    } else {
        self.data.set = {title: "", content:""};
        cb(null, null)
    }
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
            });
        } else {
            bt.button("reset");
        }
    }}, {id: self.com.get_jid("text_input"), on:"input", do:function(e){
        self.dom_preview.html(mk.to_html($(this).val()));
    }}];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_title = self.com.get("title");
    self.dom_text_input = self.com.get("text_input");
    self.dom_preview = self.com.get("preview");

    self.dom_preview.html(mk.to_html(self.dom_text_input.val()));
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    if(self.doc_id > 0) {
        data.id = self.doc_id;
    }
    data.title = self.dom_title.val();
    data.content = self.dom_text_input.val();
    return data;
}

Com.prototype.check = function(data) {
    var self = this;
    if(data.title.length == 0) {
        alert("标题不能为空");
        return false;
    }
    if(data.content.length == 0) {
        alert("内容不能为空");
        return false;
    }
    return true;
}

Com.prototype.save = function(data, cb) {
    var self = this;
    var body = {
        data:data
    }
    CurSite.postDigest({cmd:"AD02"}, body, function(err, back_body)
    {
        alert("操作成功")
        if(cb) {
            cb(null, null);
        }
    });
}

