# Extractor

Desafio:

![picture 2](../../images/7cdbbc35a1795e797537403d6f889aedb5519e447aa474d9551a1a500f2f0d82.png)  

Bien, se nos proporciona entonces una URL donde encontramos una pantalla con un input. Este input al parecer sera modificado para estar en lowercase

![picture 4](../../images/439600ed205965e18e822f30eb336dd52e7c8ffa1d1b4953bdfb6aa581460ada.png)  


![picture 3](../../images/fc56df3f97b8d0a74ebda107beaa91ce319c2677c73c2d8cc980687b2a4ce9d8.png)  

Se intento (sin exito) injecciones SQL, Template Injection, ver si podiamos acceder a algun archivo desde un metodo, etc.

Luego intentamos XSS, con `<script>alert(“xss”)</script>`

![picture 5](../../images/3d238485c4131f83f6ffe7165181420701bca587efbc21740fcf9ccce688d15f.png)  

Al parecer el `<script>` es borrado por la pagina para evitar XSS. Probamos entonces con una imagen y generando un error: `<img src=x onerror=alert(1)>`

![picture 6](../../images/9b35f7abc8daa1e0e3d79e163ba5bbbe7d923cb4bbcc1ef694dd162cb7244f59.png)  

```
Flag: shellctf{50oom3_P4yL0aDS_aM0ng_Maaa4nnY}
```