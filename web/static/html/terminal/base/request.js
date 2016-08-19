var Com = function() {
    var self = this;

    var body_ct12 = {
        ticket:
        {"bet_type":1,"game_id":202,"id":154,"number":"20160416002:00,11,33|1*1","play_type":5,"status":1,"stub":"中国体育彩票演示票\n演示票竞彩足球半全场胜平负   单场固定\n200243-945712-189922-67 DB4D0D05 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜胜@12.0元+平平@12.0元+负负@12.0元\n(选项固定奖金额为每1元投注对应的奖金额)\n本票最高可能固定奖金:268.00元\n *  *  *\n *  *  *\n─────────────────────\n倍数:1  合计:    样票  2016-04-18 12:27:08\n演示票，请勿用于销售！！！\r\n中国竞彩网 http://www.sporttery.cn\n\r\n2001021D32303032343339343537313231383939323236372030303534363835 "}
    }

    var body_ct11 = {
        id:0
    }

    var body_ct13 = {
        stub: "我是"
    }

    self.data = {
        msg: {
            body_obj:body_ct11,
            body_str:JSON.stringify(body_ct11),
            cmd: 'T11'
        },
        cmd_list: [
            {id:"T11", name:"T11"},
            {id:"T12", name:"T12"},
            {id:"T13", name:"T13"}
        ],
        body_map: {
            "T11": body_ct11,
            "T12": body_ct12,
            "T13": body_ct13,
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
            CurSite.postDigest({cmd:self.data.msg.cmd}, body, function(err, back_body)
            {
                if(err) {
                    self.dom_back_body.html(err);
                } else {
                    self.dom_back_body.html(JSON.stringify(back_body));
                }
            });
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
