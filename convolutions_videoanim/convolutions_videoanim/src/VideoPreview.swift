//
//  VideoPreview.swift
//  convolutions_videoanim
//
//  Created by Jake Landers on 3/4/23.
//

import SwiftUI

struct VideoPreview: View {
    var body: some View {
        VStack(spacing: 40) {
            Image("3b1b-whatisaconv")
                .resizable()
                .scaledToFit()
                .frame(width: 1000)
            VStack {
                Text("But What is a Convolution?")
                    .font(.system(size: 50, weight: .bold))
                Text("- 3 Blue 1 Brown on Youtube")
                    .font(.system(size: 30))
                    .opacity(0.5)
            }
        }
    }
}

struct VideoPreview_Previews: PreviewProvider {
    static var previews: some View {
        VideoPreview()
    }
}
