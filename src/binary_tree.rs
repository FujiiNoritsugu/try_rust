///
/// ノードの定義
/// 
#[derive(Debug)]
pub struct Node{
    no:i32,
    right:Box<Option<Node>>,
    left:Box<Option<Node>>
}

impl Clone for Node {
    
    fn clone(&self) -> Self {
        Node {
            no: self.no,
            right: self.right.clone(),
            left: self.left.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

impl Node{
    pub fn new(no:i32) -> Node {
        Node{
            no:no,
            right:Box::new(None),
            left:Box::new(None)
        }
    }

    pub fn get_no(&self)->i32{
        self.no
    }
}

///
/// 追加用の関数
/// 
pub fn insert(node:Box<Option<Node>>, x:i32)->Box<Option<Node>>{
    return match *node{
        None=>Box::new(Some(Node::new(x))),
        Some(v)=>{
            let mut target_node = v.clone();
            if x < target_node.no {
                target_node.left = insert(target_node.left, x);
            }else{
                target_node.right = insert(target_node.right, x);
            }
            Box::new(Some(target_node))
        }
    };
}

///
/// 検索用の関数
/// 
pub fn find(node:&Box<Option<Node>>, x:i32)->bool{
    return match &**node{
        None=>false,
        Some(v)=>{
            if x == v.no{
                true
            }else if x < v.no{
                find(&v.left, x)
            }else{
                find(&v.right, x)
            }
        }
    }
}

//
// 削除用の関数
//
pub fn remove(param:Box<Option<Node>>, x:i32)->Box<Option<Node>>{
    match *param{
        None=> Box::new(None),
        Some(v)=>{
            let mut target_node = v.clone();
            if x < target_node.no {
                target_node.left = remove(target_node.left, x);
            }else if x > target_node.no {
                target_node.right = remove(target_node.right, x);
            }else{
                //返却するノードを作成する
                let return_node = make_return_node(target_node);
                return return_node;
            }
            Box::new(Some(target_node))
        }
    }
}

///
/// 返却するノードを作成する
/// 
fn make_return_node(target_node:Node)->Box<Option<Node>>{
    let param_node = target_node.clone();

    match *(target_node.left){
        //削除したいノードが左の子を持っていない場合、右の子をもってくる
        None=>{
            make_clone(*(target_node.right))
        },
        Some(v)=>{
            // 削除したいノードの左の子が右の子を持っていなければ、左の子を持ってくる
            let mut left_node = v.clone();
            let param_left = left_node.clone();
            match *(left_node.right){
                None=>{
                    // 削除対象のノードに右の子が存在すれば、左の子の右に付け替える
                    left_node.right = make_clone(*(target_node.right));
                    Box::new(Some(left_node))
                },
                Some(_v2)=>{
                    // どちらでもなければ、左の子以下で最も大きいノードを削除したいノードの場所に持ってくる
                    search_return_node(param_node, param_left)
                }
            }
        }
    }
}

///
/// 左の子以下で最も大きいノードを削除したいノードの場所に持ってくる
/// 
fn search_return_node(target_node:Node, left_node:Node)->Box<Option<Node>>{
    // 左の子以下で最も大きいノードを探す
    let mut max_node = Some(left_node.clone());
    loop {

        match max_node.as_ref() {
            Some(v)=>{
                let right_right_node = &*(v.right);
                match right_right_node{
                    Some(v2)=>{
                        max_node = Some(v2.clone());
                    },
                    None=>{
                        break;
                    }
                }
            },
            None=>{
                break;
            }
        }
    }

    let result3 = max_node.as_mut().map(|x|{
        // この時点でtarget_nodeからmax_nodeを削除する
        let temp = Box::new(Some(target_node));
        let removed_node = (*remove(temp, x.no)).unwrap();

        x.left = make_clone(*(removed_node.left));
        x.right = make_clone(*(removed_node.right));
        (*x).clone()
    });

    Box::new(result3)

}

///
/// ノードのクローンを作成し返却する
/// 
fn make_clone(param:Option<Node>)->Box<Option<Node>>{
    match param {
        Some(v)=>Box::new(Some(v.clone())),
        None=>Box::new(None)   
    }
}

///
/// パラメータの右のノードを取得する
///  
pub fn get_right_node(param:&Box<Option<Node>>)->Box<Option<Node>>{
    match &**param {
        None=>Box::new(None),
        Some(v)=>{
            match &*v.right{
                None=>Box::new(None),
                Some(v2)=>{
                    Box::new(Some(v2.clone()))
                }
            }
        }
    }
}

///
/// パラメータの左のノードを取得する
/// 
pub fn get_left_node(param:&Box<Option<Node>>)->Box<Option<Node>>{
    match &**param {
        None=>Box::new(None),
        Some(v)=>{
            match &*v.left{
                None=>Box::new(None),
                Some(v2)=>Box::new(Some(v2.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_remove(){
        let mut checked_tree = insert(Box::new(None),7);
        checked_tree = insert(checked_tree, 2);
        checked_tree = insert(checked_tree, 1);
        checked_tree = insert(checked_tree, 5);
        checked_tree = insert(checked_tree, 4);
        checked_tree = insert(checked_tree, 15);
        checked_tree = insert(checked_tree, 10);
        checked_tree = insert(checked_tree, 8);
        checked_tree = insert(checked_tree, 11);
        checked_tree = insert(checked_tree, 17);
        checked_tree = insert(checked_tree, 16);
        checked_tree = insert(checked_tree, 19);
        checked_tree = remove(checked_tree, 15);
    
        // check 15 removed, 11, 10 exists
        assert_eq!(false, find(&checked_tree, 15));
        assert_eq!(true, find(&checked_tree, 11));
        assert_eq!(true, find(&checked_tree, 10));
    
        let right = get_right_node(&checked_tree);
        let right_left = get_left_node(&right);
        let right_left_right = get_right_node(&right_left);
  
        // check right no is 11
        assert_eq!(11, right.unwrap().no);
        // check right_left no is 10
        assert_eq!(10, right_left.unwrap().no);
        // check right_left_right is None
        assert!(right_left_right.is_none());
    }


    #[test]
    fn check_move_left_node(){
        let mut checked_tree = insert(Box::new(None),7);
        checked_tree = insert(checked_tree, 2);
        checked_tree = insert(checked_tree, 1);
        checked_tree = insert(checked_tree, 5);
        checked_tree = insert(checked_tree, 4);
        checked_tree = insert(checked_tree, 15);
        checked_tree = insert(checked_tree, 10);
        checked_tree = insert(checked_tree, 8);
        checked_tree = insert(checked_tree, 14);
        checked_tree = insert(checked_tree, 13);
        checked_tree = insert(checked_tree, 17);
        checked_tree = insert(checked_tree, 16);
        checked_tree = insert(checked_tree, 19);
        checked_tree = remove(checked_tree, 15);
    
        // check 15 removed, 13, 14 exists
        assert_eq!(false, find(&checked_tree, 15));
        assert_eq!(true, find(&checked_tree, 14));
        assert_eq!(true, find(&checked_tree, 13));
    
        let right = get_right_node(&checked_tree);
        let right_left = get_left_node(&right);
        let right_left_right = get_right_node(&right_left);
  
        // check right no is 14
        assert_eq!(14, right.unwrap().no);
        // check right_left no is 10
        assert_eq!(10, right_left.unwrap().no);
        // check right_left_right is 13
        assert_eq!(13, right_left_right.unwrap().no);
    }

}
