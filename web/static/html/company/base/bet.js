var Com = function() {
    var self = this;

    var body_ct01 = {
        tickets: [
            {game_id:200, play_type:10, bet_type:10, out_id:"0001", multiple:1, number:"01,02,03,04,05|01,12", icount:1, amount:200, term_code:2015001}
        ]
    }

    var body_ct02 = {
        id:0
    }

    var body_ct03 = {
        out_id:["0001", "0002", "0003"]
    }

    var body_ct04 = {
    }

    self.data = {
        msg: {
            body_obj:body_ct01,
            body_str:JSON.stringify(body_ct01),
            cmd: 'CT01'
        },
        cmd_list: [
            {id:"CT01", name:"CT01"},
            {id:"CT02", name:"CT02"},
            {id:"CT03", name:"CT03"},
            {id:"CT04", name:"CT04"}
        ],
        body_map: {
            "CT01": body_ct01,
            "CT02": body_ct02,
            "CT03": body_ct03,
            "CT04": body_ct04
        }
    }
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var sbt_id = self.com.get_jid("sbt");
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';
    var el = [
        {id:sbt_id, on:"click", do:function(e){
            self.data.msg.body_str = self.dom_body.val();
            self.data.msg.body_obj = JSON.parse(self.data.msg.body_str);
            var body = self.data.msg.body_obj;
            //for(var j = 0; j < 20000; j++) {
            if(self.data.msg.cmd == "CT01") {
                for(var i = 0; i < body.tickets.length; i++) {
                    var ticket = body.tickets[i];
                    ticket.out_id = CurSite.createUUID();
                }
            }
            CurSite.postDigest({cmd:self.data.msg.cmd}, body, function(err, back_body)
            {
                if(err) {
                    self.dom_back_body.html(err);
                } else {
                    self.dom_back_body.html(JSON.stringify(back_body));
                }
            });
            //}
        }},
        {id:bar_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");
            var t_id = $(this).attr("t_id");

            self.dom_body.val(JSON.stringify(self.data.body_map[t_id]));
            self.data.msg.cmd = t_id;
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_back_body = self.com.get("back_body");
    self.dom_body = self.com.get("body");
    cb(null, null)
}
