export function navigate(route, event) {
    console.log(event);
    if (event.button === 0) { //left
        window.location.assign(route);
    }
    if (event.button === 1) { //middle
        window.open(route, makeid(10));
    }

}

export function makeid(length) {
    var result           = '';
    var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var charactersLength = characters.length;
    for ( var i = 0; i < length; i++ ) {
      result += characters.charAt(Math.floor(Math.random() * 
 charactersLength));
   }
   return result;
}