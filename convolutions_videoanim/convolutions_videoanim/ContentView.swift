//
//  ContentView.swift
//  convolutions_videoanim
//
//  Created by Jake Landers on 3/3/23.
//

import SwiftUI

struct ContentView: View {
    @EnvironmentObject var model: Model
    
    var body: some View {
        Group {
            if model.show {
//                Conv2D()
//                Formula()
//                VideoPreview()
//                Pooling()
                Thumbnail()
            } else {
                VStack{}
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding(100)
        .padding(.leading, 100)
        .padding(.trailing, 100)
        
        .background(bgColor)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
