module updatePriority(input [1:0] way,input [1:0] in0,input [1:0] in1,input [1:0] in2,input [1:0] in3,
output reg [1:0] out0,output reg [1:0] out1,output reg  [1:0] out2,output reg [1:0] out3);  
  always @ (way,in0,in1,in2,in3)
  begin
    out0=in0;
    out1=in1;
    out2=in2;
    out3=in3;
    case(way)
      2'b00:  
      begin
        if(in1>in0)
          out1=in1-2'b01;
        if(in2>in0)
          out2=in2-2'b01;
        if(in3>in0)
          out3=in3-2'b01;
        out0=2'b11;
      end
      2'b01:  
      begin
        if(in0>in1)
          out0=in0-2'b01;
        if(in2>in1)
          out2=in2-2'b01;
        if(in3>in1)
          out3=in3-2'b01;
        out1=2'b11;
      end
      2'b10: 
      begin
        if(in0>in2)
          out0=in0-2'b01;
        if(in1>in2)
          out1=in1-2'b01;
        if(in3>in2)
          out3=in3-2'b01;
        out2=2'b11;
      end 
      2'b11:
      begin
        if(in0>in3)
          out0=in0-2'b01;
        if(in1>in3)
          out1=in1-2'b01;
        if(in2>in3)
          out2=in2-2'b01;
        out3=2'b11;
      end
    endcase
  end
endmodule