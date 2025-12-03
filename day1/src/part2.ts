// console.log(
//     process.argv.slice(2).reduce(([a,b],r) => {
//         d=r[0]=='R'
//         v=a+(d?1:-1)*r.slice(1);
//         m=d?.1:-.1
//         console.log(
//             a,
//             r,
//             v,
//             Math.abs(((a+m)/100^0)-((v+m)/100^0)),
//         );
//         return [
//             v,
//             b+Math.abs(((a+m)/100^0)-((v+m)/100^0))
//         ];
//     },[1000050,0])
// )

console.log(process.argv.slice(2).reduce(([a,b],r)=>(d=r[0]=='R',v=a+(d?1:-1)*r.slice(1),m=d?.1:-.1,[v,b+Math.abs(((a+m)/100^0)-((v+m)/100^0))]),[10050,0]))
