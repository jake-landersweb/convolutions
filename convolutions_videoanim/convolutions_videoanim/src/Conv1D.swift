//
//  Conv1D.swift
//  convolutions_videoanim
//
//  Created by Jake Landers on 3/3/23.
//

import SwiftUI
import Charts

struct Conv1D: View {
    private let arr1: [Double] = [0,0,0.2,0.2,0.2,0.2,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,0.2,0.2,0.2,0.2,0,0]
    private let kernel: [Double] = [-0.5,0,0.5]
    private let kernel2: [Double] = [0.1,0.2,0.4,0.2,0.1]
    @State private var out: [Double] = []
    private let horizSpacing: CGFloat = 10
    @State private var currentIndex = 0
    
    @State private var highlighted: [Int] = []
    
    var body: some View {
        GeometryReader { geo in
            VStack(spacing: 10) {
                drawGraph(x: arr1, color: accColor, alwaysLit: false, showNeg: false)
                drawGraph(x: kernel, color: Color.blue, alwaysLit: true, showNeg: true)
                    .offset(x: CGFloat(currentIndex) * (geo.size.width / CGFloat(arr1.count)))
                drawGraph(x: out, color: Color.purple, alwaysLit: true, showNeg: true)
                    .offset(x: CGFloat(2) * (geo.size.width / CGFloat(arr1.count)))
            }
        }
        .task {
            await run()
        }
    }
    
    func drawGraph(x: [Double], color: Color, alwaysLit: Bool, showNeg: Bool) -> some View {
        GeometryReader { geo in
            HStack(alignment: .bottom, spacing: horizSpacing) {
                ForEach(Array(zip(x.indices, x)), id: \.0) { index, item in
                    ZStack(alignment: .bottom) {
                        RoundedRectangle(cornerRadius: 10, style: .continuous)
                            .frame(width: geo.size.width / CGFloat(arr1.count) - horizSpacing, height: abs(geo.size.height * (item / arr1.max()!)))
                            .foregroundStyle(showNeg ? item >= 0 ? Color.green : Color.red : color)
                            .opacity(alwaysLit ? 1 : highlighted.contains(index) ? 1 : 0.5)
                        Text("\(item, specifier: "%.2f")")
                            .foregroundColor(Color.white)
                            .padding(.bottom, 5)
                    }
                }
            }
        }
        .frame(maxHeight: .infinity)
    }
    
    func run() async {
        print("RUN")
        for i in 0...(arr1.count - kernel.count) {
            // set the index
            DispatchQueue.main.async {
                withAnimation(anim) {
                    self.currentIndex = i
                }
            }
            try? await Task.sleep(nanoseconds: 1_000_000_000)
            
            // highlight
            for j in i..<i+kernel.count {
                withAnimation(anim) {
                    highlighted.append(j)
                }
            }
            
            DispatchQueue.main.async {
                var sum: Double = 0
                for j in 0..<kernel.count {
                    sum += arr1[i + j] * kernel[j]
                }
                withAnimation(anim) {
                    self.out.append(sum)
                }
            }
            try? await Task.sleep(nanoseconds: 1_000_000_000)
            withAnimation(anim) {
                highlighted = []
            }
        }
    }
}

struct Conv1D_Previews: PreviewProvider {
    static var previews: some View {
        Conv1D()
    }
}
